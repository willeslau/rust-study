struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![chars.len() as i32;chars.len()];chars.len()];

        dp[0][0] = 0;
        for i in 1..chars.len() {
            dp[i][i] = 0;
            if chars[i] == chars[i-1] {
                dp[i-1][i] = 0;
            } else {
                dp[i-1][i] = 1;
            }
        }

        for j in 3..chars.len()+1 {
            for i in 0..(chars.len()-j+1) {
                let k = i + j - 1;
                if chars[i] == chars[k] {
                    dp[i][k] = std::cmp::min(dp[i][k], dp[i+1][k-1]);
                }
                dp[i][k] = std::cmp::min(dp[i][k], dp[i+1][k] + 1);
                dp[i][k] = std::cmp::min(dp[i][k], dp[i][k-1] + 1);

                // println!("i: {}, k: {}, dp[i+1][k-1]: {}, dp[i+1][k]: {}, dp[i][k-1]: {}", i, k, dp[i+1][k-1], dp[i+1][k], dp[i][k-1]);
                // println!("dp[i][k]: {}", dp[i][k]);
            }
        }

        // println!("{:?}", dp);

        dp[0][chars.len()-1]
    }
}

#[cfg(test)]
mod tests {
    use crate::min_insertions::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::min_insertions("zzazz".to_string()),
            0
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::min_insertions("mbadm".to_string()),
            2
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::min_insertions("leetcode".to_string()),
            5
        );
    }
}