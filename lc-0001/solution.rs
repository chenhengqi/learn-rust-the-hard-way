use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        let n  = nums.len();
        for i in 0..n {
            let diff = target - nums[i];
            if m.contains_key(&diff) {
                return vec![m[&diff], i as i32];
            }
            m.insert(nums[i], i as i32);
        }
        return vec![0, (n - 1) as i32];
    }
}
