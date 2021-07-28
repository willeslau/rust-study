struct Solution;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let total_colors = cost[0].len();

        let mut dp = vec![vec![i32::MAX; (total_colors) as usize]; (target + 1) as usize];
        let mut aux = vec![vec![i32::MAX; (total_colors) as usize]; (target + 1) as usize];

        if houses[0] == 0 {
            dp[1] = cost[0].clone();
        } else {
            dp[1][houses[0] as usize-1] = 0;
        }

        println!("{:?}", dp);

        for i in 1..houses.len() {
            for k in 1..std::cmp::min(target as usize, i + 1) + 1 {
                let mut lo = 0;
                let mut up = total_colors;
                let mut already_painted = false;
                if houses[i] != 0 {
                    lo = houses[i] as usize - 1;
                    up = (lo + 1) as usize;
                    already_painted = true;
                }
                for color in lo..up {
                    let mut local_min = std::cmp::min(i32::MAX, dp[k][color]);
                    for h in 0..total_colors {
                        if h == color { continue; }
                        // aux[k][color]
                        local_min = std::cmp::min(local_min, dp[k-1][h]);
                    }
                    if local_min == i32::MAX || already_painted {
                        aux[k][color] = local_min;
                    } else {
                        aux[k][color] = local_min + cost[i][color];
                    }
                }
            }
            std::mem::swap(&mut dp, &mut aux);
            println!("{:?}", dp);
        }

        let mut min = i32::MAX;
        for i in 0..total_colors {
            min = std::cmp::min(min, dp[target as usize][i]);
        }

        if min == i32::MAX {
            return -1;
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use crate::paint_house_iii::Solution;

    #[test]
    fn test_case1() {
        let house = vec![0,0,0,0,0];
        let cost = [[1,10],[10,1],[10,1],[1,10],[5,1]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 3), 9);
    }

    #[test]
    fn test_case3() {
        let house = vec![0,0,0,0,0];
        let cost = [[1,10],[10,1],[1,10],[10,1],[1,10]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 5), 5);
    }

    #[test]
    fn test_case2() {
        let house = vec![0,2,1,2,0];
        let cost = [[1,10],[10,1],[10,1],[1,10],[5,1]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 3), 11);
    }

    #[test]
    fn test_case4() {
        let house = vec![3,1,2,3];
        let cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 3), -1);
    }

    #[test]
    fn test_case5() {
        let house = vec![2,3,0];

        let cost = [[5,2,3],[3,4,1],[1,2,1]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 3), 1);
    }

    #[test]
    fn test_case6() {
        let house = vec![0,1,0,0,1,2,0,0,2,1];
        let cost = [[4,5,2,6],[8,3,2,9],[6,7,3,1],[10,10,2,7],[6,5,2,4],[4,4,3,9],[9,8,3,5],[7,9,10,3],[8,5,9,10],[10,7,4,6]]
            .map(|a| a.to_vec())
            .to_vec();
        assert_eq!(Solution::min_cost(house.clone(), cost.clone(), house.len() as i32, cost[0].len() as i32, 6), 24);
    }
}
