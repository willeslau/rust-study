use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {

        // dp[i][j] = max(
        //      dp[i][j],
        //      if prefix[i][k] != dp[k+2][j] => min(dp[i][k], dp[k+1][j])
        //      else max(dp[i][k], dp[k+1][j]
        // )
        let mut dp = vec![vec![0;stone_value.len()];stone_value.len()];
        let mut prefix_sum = vec![0;stone_value.len()+1];

        for i in 0..stone_value.len() {
            prefix_sum[i+1] = prefix_sum[i] + stone_value[i];
        }

        for len in 2..stone_value.len()+1 {
            for i in 0..stone_value.len()-len+1 {
                let j = i + len - 1;
                // TODO: this inner loop can be optimized to O(1)
                for k in i..j {
                    let sum_left = prefix_sum[k+1] - prefix_sum[i];
                    let sum_right = prefix_sum[j+1] - prefix_sum[k+1];

                    if sum_left == sum_right {
                        dp[i][j] = max(
                            dp[i][j],
                            max(dp[i][k] + sum_left, dp[k+1][j] + sum_right)
                        );
                    } else if sum_left > sum_right {
                        dp[i][j] = max(
                            dp[i][j],
                            dp[k+1][j] + sum_right
                        );
                    } else {
                        dp[i][j] = max(
                            dp[i][j],
                            dp[i][k] + sum_left
                        );
                    }

                }
            }
        }
        dp[0][stone_value.len()-1]
    }
}


#[cfg(test)]
mod tests {
    use crate::stone_game_v::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::stone_game_v([6,2,3,4,5,5].to_vec()),
            18
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::stone_game_v([7,7,7,7,7,7,7].to_vec()),
            28
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::stone_game_v([7].to_vec()),
            0
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::stone_game_v([6,2,3].to_vec()),
            7
        );
    }
}