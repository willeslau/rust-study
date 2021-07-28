struct Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let row = costs.len();
        let col = costs[0].len();
        let mut dp = vec![i32::MAX; col];
        let mut aux = vec![i32::MAX; col];

        for i in 0..col { dp[i] = costs[0][i]; }

        // can optimize such that we track the min 2. If the min is taken, we use the
        // second smallest directly.
        for i in 1..row {
            for j in 0..col {
                let mut min = i32::MAX;
                for k in 0..j as i32{
                    min = std::cmp::min(min, costs[i][j] + dp[k as usize]);
                }
                for k in j+1..col {
                    min = std::cmp::min(min, costs[i][j] + dp[k]);
                }
                aux[j] = min;
            }
            std::mem::swap(&mut dp, &mut aux);
        }

        // println!("{:?}", dp);
        let mut min = i32::MAX;
        for k in 0..col {
            min = std::cmp::min(min, dp[k]);
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use crate::paint_house_i::Solution;

    #[test]
    fn test_case1() {
        let costs = [[17,2,17],[16,16,5],[14,3,19]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(costs), 10);
    }

    #[test]
    fn test_case2() {
        let costs = [[7,6,2]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(costs), 2);
    }
}
