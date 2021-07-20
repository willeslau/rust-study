struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut prefix_sum = vec![0;nums.len()+1];
        for i in 0..nums.len() { prefix_sum[i+1] = prefix_sum[i] + nums[i]; }

        // main loop
        let mut lo = 1;
        let mut hi = nums.len();
        let mut result = 0;
        while lo <= hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::check(&nums, &prefix_sum, mid, k) {
                // println!("lo: {}, {}, {}, true", lo, mid, hi);
                lo = mid + 1;
            } else {
                // println!("lo: {}, {}, {}, false", lo, mid, hi);
                hi = mid - 1;
            }
        }

        hi as i32
    }

    fn check(nums: &Vec<i32>, prefix_sum: &Vec<i32>, len: usize, k: i32) -> bool {
        for i in len-1..nums.len() {
            let sum = prefix_sum[i+1] - prefix_sum[i+1-len];
            // println!("i: {}, nums[i] * len: {}, len: {}, sum + k: {}", i, nums[i] * len as i32, len, sum + k);
            if nums[i] * len as i32 <= sum + k { return true; }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::max_frequency::Solution;

    #[test]
    fn it_works() {
        let v = vec![1,4,8,13];
        let k = 5;
        assert_eq!(
            Solution::max_frequency(v, k),
            2
        );
    }

    #[test]
    fn test_case2() {
        let v = vec![1,2,4];
        let k = 5;
        assert_eq!(
            Solution::max_frequency(v, k),
            3
        );
    }

    #[test]
    fn test_case3() {
        let v = vec![3,9,6];
        let k = 2;
        assert_eq!(
            Solution::max_frequency(v, k),
            1
        );
    }
}