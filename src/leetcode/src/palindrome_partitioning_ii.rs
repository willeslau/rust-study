struct Solution {}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut is_palindrome = vec![vec![false;s.len()];s.len()];
        let chars: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            is_palindrome[i][i] = true;
        }

        for i in (0..s.len()).rev() {
            for j in i+1..s.len() {
                is_palindrome[i][j] = chars[i] == chars[j];
                if j != i+1 {
                    is_palindrome[i][j] &= is_palindrome[i+1][j-1];
                }
            }
        }

        let mut dp = vec![chars.len();chars.len()+1];
        dp[0] = 0;
        dp[1] = 1;
        for i in 1..chars.len() {
            let k = i+1;
            for j in (0..i+1).rev() {
                if !is_palindrome[j][i] {
                    dp[k] = std::cmp::min(dp[k], dp[j] + i - j + 1);
                } else {
                    dp[k] = std::cmp::min(dp[k], dp[j] + 1);
                }
            } 
        }

        *dp.get(chars.len()).unwrap() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome_partitioning_ii::Solution;

    #[test]
    fn it_works() {
        let s = "aaaaaab";
        assert_eq!(Solution::min_cut(s.to_string()), 1);
    }
}