struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; (min_profit + 1) as usize]; (n + 1) as usize];
        let mut aux = vec![vec![0; (min_profit + 1) as usize]; (n + 1) as usize];

        aux[0][0] = 1;
        aux[group[0] as usize][profit[0] as usize] = 1;

        for index in 1..profit.len() {
            for i in 0..(n as usize + 1) {
                for j in 0..(min_profit as usize + 1) {
                    if i < group[index] as usize {
                        dp[i][j] = aux[i][j];
                    } else {
                        dp[i][j] = aux[i][j] + dp[i - (group[index] as usize)][std::cmp::max(0, j as i32 - profit[index] ) as usize];
                    }
                }
            }
            println!("aux: {:?}", aux);
            std::mem::swap(&mut dp, &mut aux);
            println!("{:?}", dp);
        }


        let mut result = 0;
        for i in 1..n as usize + 1 {
            result += dp[i][min_profit as usize];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::profitable_schemes::Solution;

    #[test]
    pub fn case_1() {
        let n = 5;
        let min_profit = 3;
        let group = vec![2, 2];
        let profit = vec![2, 3];
        assert_eq!(Solution::profitable_schemes(n, min_profit, group, profit), 2);
    }
}