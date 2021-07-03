use std::ptr;
use std::marker::PhantomData;
use std::cmp::{Ord, Ordering};
use crate::Sorting;

/// When the number of elements is small, switch to insertion sort improves the performance.
/// However, when compared with Rust std lib, this is too slow. One way to enhance this 
/// maybe is the recursion? 
#[derive(Debug)]
pub struct MergeSort<T: Ord + Clone>{
    phantom: PhantomData<T>
}

impl <T> Sorting<T> for MergeSort<T> where T: Ord + Clone {
    fn sort(data: &mut Vec<T>) {
        let mut aux = data.clone();
        Self::merge_sort(data, &mut aux, 0, data.len()-1);

    }
}

impl <T> MergeSort<T> where T: Ord + Clone {
    fn merge_sort(data: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize) {
        if lo == hi { return; }
        if hi - lo < 20 {
            Self::insertion_sort(data, lo, hi);
            return;
        }
        let mid = (hi - lo) / 2 + lo;

        Self::merge_sort(data, aux, lo, mid);
        Self::merge_sort(data, aux, mid+1, hi);
        Self::merge(data, aux, lo, mid, hi);
    }

    fn merge(data: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
        let mut i = lo;
        let mut j = mid + 1;
        
        for k in lo..(hi+1) {
            if i > mid {
                Self::assign(data, j, aux, k);
                j += 1;
            } else if j > hi {
                Self::assign(data, i, aux, k);
                i += 1
            } else {
                match data[i].cmp(&data[j]) {
                    Ordering::Greater => {
                        Self::assign(data, j, aux, k);
                        j += 1;
                    },
                    _ => {
                        Self::assign(data, i, aux, k);
                        i += 1;
                    }
                }
            }
        }

        for k in lo..(hi+1) { Self::assign(aux, k, data, k); }
    }

    /// b[j] = a[i]
    fn assign(a: &mut Vec<T>, i: usize, b: &mut Vec<T>, j: usize) {
        unsafe {
            let a_ptr: *const T = &a[i];
            let b_ptr: *mut T = &mut b[j];
            ptr::copy_nonoverlapping(a_ptr, b_ptr, 1);
        }
    }

    fn insertion_sort(data: &mut Vec<T>, lo: usize, hi: usize) {
        if hi - lo < 2 { return; }

        for i in lo+1..hi+1 {
            for j in (lo..i).rev() {
                match data[j+1].cmp(&data[j]) {
                    Ordering::Less => Self::swap(data, j, j+1),
                    Ordering::Greater => break,
                    _ => {}
                }
            }
        }
    }

    fn swap(data: &mut Vec<T>, i: usize, j: usize) {
        unsafe {
            let ptr_a: *mut T = &mut data[i];
            let ptr_b: *mut T = &mut data[j];
            ptr::swap(ptr_a, ptr_b);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Sorting;
    use crate::sorting::merge::MergeSort;
    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let mut v = vec![5,4,3,2,4,5,6,7];
        MergeSort::sort(&mut v);
        assert_eq!(v, vec![2, 3, 4, 4, 5, 5, 6, 7]);        
    }

    #[test]
    fn speed() {
        let mut rng = rand::thread_rng();
        let v: Vec<u64> = (0..100000).map(|_| rng.gen_range(0, 1000)).collect();

        let mut f = v.clone();
        let start = Instant::now();
        MergeSort::sort(&mut f);
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());

        let mut f = v.clone();
        let start = Instant::now();
        f.sort();
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
    }
}