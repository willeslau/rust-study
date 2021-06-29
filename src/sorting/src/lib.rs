mod insert;
pub use insert::InsertionSort;

pub trait Sorting<T> where T: Ord {
	fn sort(data: &mut Vec<T>);
}
