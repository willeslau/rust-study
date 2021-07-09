mod sorting;
mod linkedlist;

use std::cmp::Ord;
pub use linkedlist::List;

/// The sorting trait
pub trait Sorting<T> where T: Ord {
    fn sort(data: &mut Vec<T>);
}
