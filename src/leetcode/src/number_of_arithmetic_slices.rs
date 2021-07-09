use std::cell::RefCell;
use std::collections::HashMap;

struct Solution;


impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        // We need RefCell here because we are mutating dp, see below for more
        let mut dp: Vec<RefCell<HashMap<i32, i32>>> = vec![RefCell::new(HashMap::new()); nums.len()];

        let mut total = 0;
        for i in 1..nums.len() {
            // Here we need a mutable reference to dp
            let mut i_map = dp.get(i).unwrap().borrow_mut();

            for j in 0..i {
                let diff = nums[i] - nums[j];
                // But here we also have a immutable reference after we obtained a mutable reference
                // From the compiler point of view, the i_map might have removed the index at j_map's
                // index. With this, then j_map might no longer be safe. For the dev, we know this is
                // not happening, RefCell will have us do it.
                let j_map = dp.get(j).unwrap().borrow();

                let count = *j_map.get(&diff).unwrap_or(&0);
                let origin = *i_map.get(&diff).unwrap_or(&0);

                i_map.insert(diff, count + origin + 1);
                total += count;
            }
        }

        total
    }
}


#[cfg(test)]
mod tests {
    use crate::number_of_arithmetic_slices::Solution;

    #[test]
    pub fn number_of_arithmetic_slices_case_1() {
        let v = vec![2, 4, 6, 8, 10];
        // 2:  {}
        // 4:  {2: -1}
        // 6:  {2: 1, 4: -1}
        // 8:  {2: 2, 4: -1, 6: -1}
        // 10: {2:
        assert_eq!(Solution::number_of_arithmetic_slices(v), 7);
    }
}