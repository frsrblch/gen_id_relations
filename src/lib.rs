use force_derive::{ForceClone, ForceDefault};
use gen_id_allocator::{Id, IdRange, ValidId};
use gen_id_component::RawComponent;
use iter_context::ContextualIterator;
use std::ops::Index;

mod range;
mod vec;

pub use range::RangeRelations;
pub use vec::VecRelations;
