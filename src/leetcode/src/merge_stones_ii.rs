// url: https://leetcode-cn.com/problems/minimum-cost-to-merge-stones/

struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![i32::MAX;k + 1];stones.len()];stones.len()];
        let mut prefix_sum = vec![0];
        for i in 0..stones.len() {
            prefix_sum.push(prefix_sum[i] + stones[i]);
        }

        for i in 0..stones.len() {
            dp[i][i][1] = 0;
        }

        for len in 2..stones.len()+1 {
            println!("{}", len);
            for i in 0..stones.len()-len+1 {
                let j = i + len - 1;
                for f in i..j {
                    for h in 2..k+1 {
                        // println!("i: {}, f: {}, j: {}, h:{}, len: {}, bool: {}", i, f, j, k, len, (len - 1) % (h - 1) != 0);
                        // if (len - 1) % (k - 1) != 0 { continue; }
                        if dp[i][f][1] == i32::MAX || dp[f+1][j][h-1] == i32::MAX { continue; }
                        dp[i][j][h] = std::cmp::min(
                            dp[i][j][h],
                            dp[i][f][1] + dp[f+1][j][h-1]
                        );
                    }
                    if j + 1 - i < k || dp[i][j][k] == i32::MAX { continue; }
                    // println!("i: {}, j: {}, k:{}, dp[i][j][k]: {}", i, j, k, dp[i][j][k]);
                    dp[i][j][1] = dp[i][j][k] + prefix_sum[j+1] - prefix_sum[i];
                }
                println!("i: {}, j: {}, result: {:?}", i, j, dp[i][j]);
            }
        }

        if dp[0][stones.len()-1][1] == i32::MAX {
            return -1;
        }
        dp[0][stones.len()-1][1]
    }
}


#[cfg(test)]
mod tests {
    use crate::merge_stones_ii::Solution;

    #[test]
    fn it_works() {
        let stones = [3,2,4,1].to_vec();
        let k = 2;
        assert_eq!(Solution::merge_stones(stones, k), 20);
    }

    #[test]
    fn testcase_2() {
        let stones = [3,2,4,1].to_vec();
        let k = 3;
        assert_eq!(Solution::merge_stones(stones, k), -1);
    }

    #[test]
    fn testcase_3() {
        let stones = [3,5,1,2,6].to_vec();
        let k = 3;
        assert_eq!(Solution::merge_stones(stones, k), 25);
    }

    #[test]
    fn testcase_4() {
        let stones = [25,68,35,62,52,57,35,83,40,51,30,20,51].to_vec();
        let k = 5;
        assert_eq!(Solution::merge_stones(stones, k), 1042);
    }
}