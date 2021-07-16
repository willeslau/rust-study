struct Solution;

/// link: https://leetcode-cn.com/problems/minimum-absolute-sum-difference/
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut num1 = nums1.clone();
        let len = nums1.len();

        let mut abs = vec![0; len];
        for i in 0..nums1.len() {
            abs[i] = (*&nums1[i] - *&nums2[i]).abs();
        }

        let mut sum = 0;
        for i in &abs {
            sum += *i;
            sum = sum % 1000000007
        }
        if sum == 0 { return 0; }

        num1.sort();

        let mut max_deduct = 0;
        for i in 0..len {
            let n = &nums2[i];
            match num1.binary_search(n) {
                Ok(j) => {
                    max_deduct = std::cmp::max(max_deduct, abs[i]);
                },
                Err(j) => {
                    if j == nums2.len() {
                        max_deduct = std::cmp::max(max_deduct, abs[i] - (num1[j-1] - *n).abs());
                    } else if j > 0 {
                        max_deduct = std::cmp::max(max_deduct, abs[i] - (num1[j-1] - *n).abs());
                        max_deduct = std::cmp::max(max_deduct, abs[i] - (num1[j] - *n).abs());
                    }
                    else {
                        max_deduct = std::cmp::max(max_deduct, abs[i] - (num1[j] - *n).abs());
                    }
                }
            }
        }

        sum = sum - max_deduct % 1000000007;
        if sum < 0 {
            return sum + 1000000007;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::min_absolute_sum_diff::Solution;

    #[test]
    fn test_case_1() {
        let nums1 = vec![1,7,5];
        let nums2 = vec![2,3,5];

        assert_eq!(
            Solution::min_absolute_sum_diff(nums1, nums2),
            3
        );
    }

    #[test]
    fn test_case_2() {
        let nums1 = [2,4,6,8,10];
        let nums2 = [2,4,6,8,10];

        assert_eq!(
            Solution::min_absolute_sum_diff(Vec::from(nums1), Vec::from(nums2)),
            0
        );
    }

    #[test]
    fn test_case_3() {
        let nums1 = [1,10,4,4,2,7];
        let nums2 = [9,3,5,1,7,4];

        assert_eq!(
            Solution::min_absolute_sum_diff(Vec::from(nums1), Vec::from(nums2)),
            20
        );
    }

    #[test]
    fn test_case_4() {
        let nums1 = [56,51,39,1,12,14,58,82,18,41,70,64,18,7,44,90,55,23,11,79,59,76,67,92,60,80,57,11,66,32,76,73,35,65,55,37,38,26,4,7,64,84,98,61,78,1,80,33,5,66,32,30,52,29,41,2,21,83,30,35,21,30,13,26,36,93,81,41,98,23,20,19,45,52,25,51,52,24,2,45,21,97,11,92,28,37,58,29,5,18,98,94,86,65,88,8,75,12,9,66];
        let nums2 = [64,32,98,65,67,40,71,93,74,24,49,80,98,35,86,52,99,65,15,92,83,84,80,71,46,11,26,70,80,2,81,57,97,12,68,10,49,80,24,18,45,72,33,94,60,5,94,99,14,41,25,83,77,67,49,70,94,83,55,17,61,44,50,62,3,36,67,10,2,39,53,62,44,72,66,7,3,6,80,38,43,100,17,25,24,78,8,4,36,86,9,68,99,64,65,15,42,59,79,66];

        assert_eq!(
            Solution::min_absolute_sum_diff(Vec::from(nums1), Vec::from(nums2)),
            3029
        );
    }

    #[test]
    fn test_case_5() {
        let nums1 = [1,7,5];
        let nums2 = [2,3,5];

        assert_eq!(
            Solution::min_absolute_sum_diff(Vec::from(nums1), Vec::from(nums2)),
            3
        );
    }

    #[test]
    fn test_case_6() {
        let nums1 = [53,48,14,71,31,55,6,80,28,19,15,40,7,21,69,15,5,42,86,15,11,54,44,62,9,100,2,26,81,87,87,18,45,29,46,100,20,87,49,86,14,74,74,52,52,60,8,25,21,96,7,90,91,42,32,34,55,20,66,36,64,67,44,51,4,46,25,57,84,23,10,84,99,33,51,28,59,88,50,41,59,69,59,65,78,50,78,50,39,91,44,78,90,83,55,5,74,96,77,46];
        let nums2 = [39,49,64,34,80,26,44,3,92,46,27,88,73,55,66,10,4,72,19,37,40,49,40,58,82,32,36,91,62,21,68,65,66,55,44,24,78,56,12,79,38,53,36,90,40,73,92,14,73,89,28,53,52,46,84,47,51,31,53,22,24,14,83,75,97,87,66,42,45,98,29,82,41,36,57,95,100,2,71,34,43,50,66,52,6,43,94,71,93,61,28,84,7,79,23,48,39,27,48,79];

        assert_eq!(
            Solution::min_absolute_sum_diff(Vec::from(nums1), Vec::from(nums2)),
            3156
        );
    }
}