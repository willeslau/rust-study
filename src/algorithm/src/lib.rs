mod sorting;
mod linkedlist;

use std::cmp::Ord;

/// The sorting trait
pub trait Sorting<T> where T: Ord {
    fn sort(data: &mut Vec<T>);
}
