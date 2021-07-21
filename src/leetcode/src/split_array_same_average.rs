struct Solution;

/// ere is a slight "half-ing" optimization which would speed up a lot!
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let n = nums.len() as i32;
        let mut target = 0;
        for i in 0..nums.len() {
            target += nums[i];
            nums[i] *= n;
        }

        // println!("{:?}", nums);
        nums.sort();
        Self::dfs(&nums, target, 0, 0, 0)
    }

    fn dfs(nums: &Vec<i32>, target: i32, sum_now: i32, count: usize, index: usize) -> bool {
        let n = target * count as i32;
        // println!("n: {}, sum_now: {}", n, sum_now);
        if sum_now != 0 && count != nums.len() && n == sum_now { return true; }
        if sum_now > n || index == nums.len() { return false; }

        if Self::dfs(nums, target, sum_now + nums[index], count+1, index+1) {
            return true;
        }
        return Self::dfs(nums, target, sum_now, count, index+1);
    }
}

#[cfg(test)]
mod tests {
    use crate::split_array_same_average::Solution;

    #[test]
    fn test_case1() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            Solution::split_array_same_average(v),
            true
        );
    }

    #[test]
    fn test_case2() {
        let v = vec![4, 2, 3];
        assert_eq!(
            Solution::split_array_same_average(v),
            true
        );
    }

    #[test]
    fn test_case3() {
        let v = vec![2, 3];
        assert_eq!(
            Solution::split_array_same_average(v),
            false
        );
    }
}