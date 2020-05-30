impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut i = 0;
        for j in 1..n {
            if nums[j] != nums[i] {
                nums[i + 1] = nums[j];
                i += 1;
            }
        }
        (i + 1) as i32
    }
}
