// // 将n堆石子绕圆形操场排放，现要将石子有序地合并成一堆。规定每次只能选相邻的两堆合并成新的一堆，并将新的一堆的石子数记做该次合并的得分。
// //
// // 请编写一个程序，读入堆数n-1及每堆的石子数，并进行如下计算：
// // 选择一种合并石子的方案，使得做  次合并得分总和最大。
// // 选择一种合并石子的方案，使得做  次合并得分总和最小。
//
// struct Solution;
//
// impl Solution {
//     pub fn merge_stones(stones: Vec<u32>) -> (u32, u32) {
//         let mut min_dp = vec![vec![u32::MAX; stones.len()*2]; stones.len()*2];
//         let mut max_dp = vec![vec![u32::MIN; stones.len()*2]; stones.len()*2];
//
//         let mut modified_stones = vec![];
//         for n in stones { modified_stones.push(n); }
//         for n in stones { modified_stones.push(n); }
//
//         let mut prefix_sum = vec![0];
//         for i in 0..modified_stones.len() {
//             prefix_sum[i+1] = prefix_sum[i] + modified_stones[i];
//         }
//
//         for len in 2..stones.len()+1 {
//             for i in 0..modified_stones.len() - len {
//                 let j = i + len - 1;
//                 for k in i..j {
//                     min_dp[i][j] = std::cmp::min(
//                         min_dp[i][j],
//                         min_dp[i][k] + min_dp[k+1][j] + prefix_sum[j+1] - prefix_sum[i]
//                     );
//                     max_dp[i][j] = std::cmp::max(
//                         max_dp[i][j],
//                         max_dp[i][k] + max_dp[k+1][j] + prefix_sum[j+1] - prefix_sum[i]
//                     );
//                 }
//             }
//         }
//
//         let mut min = u32::MAX;
//         let mut max = u32::MIN;
//         for i in 0..stones.len() {
//             min = std::cmp::min(min, min_dp[i][i+stones.len()-1]);
//             max = std::cmp::min(max, max_dp[i][i+stones.len()-1]);
//         }
//
//         (min, max)
//     }
// }