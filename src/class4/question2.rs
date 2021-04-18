pub fn sum(nums: &[u32]) -> Option<u32> {
    let mut s: u32 = 0;
    for i in nums {
        if let Some(k) = s.checked_add(*i) { s = k; }
        else { return None; }
    }
    Some(s)
}