
struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i32 {
        if Self::gcd(nums[0], nums[nums.len()-1]) != 1i32 { return 1; }

        let mut cache = vec![-1;nums.len()];
        Self::split(0, &nums, &mut cache)
    }

    fn split(i: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if i == nums.len() { return 0; }
        if i == nums.len()-1 { return 1; }
        if cache[i] != -1 { return cache[i]; }

        let mut min = i32::MAX;
        for j in (i..nums.len()).rev() {
            if Self::gcd(nums[i], nums[j]) != 1 {
                min = std::cmp::min(min, 1 + Self::split(j+1, nums, cache));
            }
        }

        cache[i] = min;

        min

    }

    fn gcd(mut aa: i32, mut bb: i32) -> i32 {
        while bb > 0 {
            let tmp = aa;
            aa = bb;
            bb = tmp % bb;
        }
        aa
    }
}

#[cfg(test)]
mod tests {
    use crate::split_array::Solution;

    #[test]
    fn test_case1() {
        let v = vec![2, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            Solution::split_array(v),
            1
        );
    }

    #[test]
    fn test_case2() {
        let v = vec![2,3,3,2,3,3];
        assert_eq!(
            Solution::split_array(v),
            2
        );
    }

    #[test]
    fn test_case3() {
        let v = vec![2,3,5,7];
        assert_eq!(
            Solution::split_array(v),
            4
        );
    }

    #[test]
    fn test_case4() {
        let v = vec![197597,26083,231529,216133,22483,74411,89087,218681,863,228421,214463,224863,5737,32941,216103,132689,159737,151241,164309,73643,45121,59981,68821,11197,54679,85213,138727,89657,102769,112121,136573,27059,77351,109891,94229,173617,224443,149531,84979,31013,219409,156749,108233,80107,90173,138899,151057,66683,66683,153911,69959,79451,75407,159319,7411,78571,128717,52057,55799,128201,125353,214763,12071,152657,81199,190391,96779,62659,27997,318559,299113,258691,258031,296713,297533,341477,273271,270659,296479,262693,270287,247769,246781,308509,289031,298559,246439,318713,317773,260879,322237,245851,276623,319237,352589,283463,235111,393203,917327,495371];
        assert_eq!(
            Solution::split_array(v),
            99
        );
    }
}