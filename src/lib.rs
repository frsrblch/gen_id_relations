use force_derive::{ForceClone, ForceCopy, ForceDefault, ForceEq, ForcePartialEq};
use gen_id::{component::RawComponent, Id, IdRange, ValidId};
use iter_context::ContextualIterator;
use std::ops::Index;

mod range;
mod vec;

pub use range::*;
pub use vec::*;
