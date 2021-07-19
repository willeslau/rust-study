struct Solution;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let mut dp = vec![vec![0;s.len()];s.len()];
        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        let chars: Vec<char> = s.chars().collect();
        for size in 2..chars.len()+1 {
            for i in 0..chars.len() - size + 1 {
                let j = i + size - 1;
                if chars[i] != chars[j] {
                    dp[i][j] = dp[i+1][j] + dp[i][j-1] - dp[i+1][j-1];
                } else {
                    dp[i][j] = 2 * dp[i+1][j-1];

                    let mut r = j - 1;
                    let mut l = i + 1;
                    while l <= r && chars[r] != chars[j] { r -= 1; }
                    while l <= r && chars[l] != chars[j] { l += 1; }

                    if l < r { dp[i][j] -= dp[l + 1][r - 1]; }
                    else if l == r { dp[i][j] += 1; }
                    else { dp[i][j] += 2; }
                }

                if dp[i][j] < 0 {
                    dp[i][j] += 1000000007;
                } else {
                    dp[i][j] %= 1000000007;
                }
            }
        }

        dp[0][s.len()-1]
    }
}

#[cfg(test)]
mod tests {
    use crate::count_palindromic_subsequences::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::count_palindromic_subsequences("bccb".to_ascii_lowercase()),
            6
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::count_palindromic_subsequences("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_ascii_lowercase()),
            104860361
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::count_palindromic_subsequences("a".to_ascii_lowercase()),
            1
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::count_palindromic_subsequences("bcbacbabdcbcbdcbddcaaccdcbbcdbcabbcdddadaadddbdbbbdacbabaabdddcaccccdccdbabcddbdcccabccbbcdbcdbdaada".to_ascii_lowercase()),
            117990582
        );
    }
}