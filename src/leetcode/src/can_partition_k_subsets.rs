use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 { return false; }

        let avg = sum / k;
        let mut nums = nums;
        nums.sort();
        Self::can_partition(&nums, avg, 0, k, &mut vec![false;nums.len()], 0)
    }

    fn can_partition(nums: &Vec<i32>, target: i32, sum_now: i32, k: i32, used: &mut Vec<bool>, begin: usize) -> bool {
        if k == 1 { return true; }

        // reason being no more summation needed as the sum would only get bigger
        if sum_now == target {
            // note that begin here starts at 0
            return Self::can_partition(nums, target, 0, k-1, used, 0);
        }

        if sum_now > target { return false; }

        for i in begin..nums.len() {
            if used[i] { continue; }

            let n = nums[i] + sum_now;
            used[i] = true;
            // no need to check previous index anymore
            if Self::can_partition(nums, target, n, k, used, begin+1) {
                return true;
            }
            used[i] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::can_partition_k_subsets::Solution;

    #[test]
    fn test_intersect_1() {
        let nums = [4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        assert_eq!(Solution::can_partition_k_subsets(Vec::from(nums), k), true);
    }

    #[test]
    fn test_intersect_2() {
        let nums = [4,3,2,3,5,2,1,4];
        let k = 4;
        assert_eq!(Solution::can_partition_k_subsets(Vec::from(nums), k), true);
    }

    #[test]
    fn test_intersect_3() {
        let nums = [4,3,2,3,5,2,1,7,6,5,4,4,56,6,9,9];
        let k = 7;
        assert_eq!(Solution::can_partition_k_subsets(Vec::from(nums), k), true);
    }
}