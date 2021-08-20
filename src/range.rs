use super::*;
use gen_id_allocator::Fixed;

#[derive(Debug, ForceClone)]
pub enum RangeRelation<Arena> {
    ChildOf(Id<Arena>),
    ParentOf(IdRange<Arena>),
}

impl<Arena> PartialEq for RangeRelation<Arena> {
    fn eq(&self, other: &Self) -> bool {
        use RangeRelation::*;
        match (self, other) {
            (ChildOf(lhs), ChildOf(rhs)) => lhs.eq(rhs),
            (ParentOf(lhs), ParentOf(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl<Arena> Eq for RangeRelation<Arena> {}

impl<Arena> RangeRelation<Arena> {
    #[inline]
    pub fn parent() -> Self {
        Self::ParentOf(IdRange::default())
    }
}

#[derive(Debug, ForceDefault, ForceClone)]
pub struct RangeRelations<Arena> {
    values: RawComponent<Arena, RangeRelation<Arena>>,
}

/// Requires fixed because unlinking is not implemented
impl<Arena: Fixed> RangeRelations<Arena> {
    #[inline]
    fn insert_if_empty(&mut self, id: impl ValidId<Arena = Arena>, relation: RangeRelation<Arena>) {
        match self.values.get(id.id()) {
            None => self.values.insert(id.id(), relation),
            Some(_existing) => panic!(
                "{}::insert_if_empty: cannot insert over existing relation",
                std::any::type_name::<Self>()
            ),
        }
    }

    #[inline]
    pub fn insert_parent(&mut self, id: impl ValidId<Arena = Arena>) {
        self.insert_if_empty(id, RangeRelation::parent());
    }

    #[inline]
    pub fn insert_child<V0: ValidId<Arena = Arena>, V1: ValidId<Arena = Arena>>(
        &mut self,
        id: V0,
        parent: V1,
    ) {
        match &mut self.values[parent.id()] {
            RangeRelation::ParentOf(children) => children.extend(id.id()),
            _ => panic!("parent id is not a parent"),
        }

        let relation = RangeRelation::ChildOf(parent.id());
        self.insert_if_empty(id, relation);
    }
}

impl<Arena, V: ValidId<Arena = Arena>> Index<V> for RangeRelations<Arena> {
    type Output = RangeRelation<Arena>;

    #[inline]
    fn index(&self, index: V) -> &Self::Output {
        self.values.index(index.id())
    }
}

impl<'a, Arena> IntoIterator for &'a RangeRelations<Arena> {
    type Item = &'a RangeRelation<Arena>;
    type IntoIter = <&'a RawComponent<Arena, RangeRelation<Arena>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, Arena> ContextualIterator for &'a RangeRelations<Arena> {
    type Context = Arena;
}

#[cfg(test)]
mod test {
    use super::*;
    use gen_id_allocator::{fixed_id, Valid};

    #[derive(Debug)]
    struct Arena;

    fixed_id! { Arena }

    fn get_id<Arena>(index: usize) -> Valid<'static, Id<Arena>> {
        Valid::assert(Id::<Arena>::first(index))
    }

    #[test]
    fn get_children_for_new_parent_returns_empty_vec() {
        let mut graph = RangeRelations::<Arena>::default();

        let parent = get_id(0);

        graph.insert_parent(parent);
        assert_eq!(graph[parent], RangeRelation::parent());
    }

    #[test]
    fn link_child_to_parent() {
        let mut graph = RangeRelations::<Arena>::default();

        let id0 = get_id(0);
        let id1 = get_id(1);

        graph.insert_parent(id0);
        graph.insert_child(id1, id0);

        assert_eq!(graph[id0], RangeRelation::ParentOf(IdRange::from(id1.id())));
        assert_eq!(graph[id1], RangeRelation::ChildOf(id0.id()));
    }

    #[test]
    #[should_panic]
    fn link_child_to_another_child() {
        let mut graph = RangeRelations::<Arena>::default();

        let id0 = get_id(0);
        let id1 = get_id(1);
        let id2 = get_id(2);

        graph.insert_parent(id0);
        graph.insert_child(id1, id0);
        graph.insert_child(id2, id1);
    }

    #[test]
    #[should_panic]
    fn insert_parent_overtop_of_another_link() {
        let mut graph = RangeRelations::<Arena>::default();

        let id0 = get_id(0);

        graph.insert_parent(id0);
        graph.insert_parent(id0);
    }

    #[test]
    #[should_panic]
    fn insert_child_overtop_of_another_parent() {
        let mut graph = RangeRelations::<Arena>::default();

        let id0 = get_id(0);
        let id1 = get_id(1);

        graph.insert_parent(id0);
        graph.insert_parent(id1);
        graph.insert_child(id1, id0);
    }
}
