impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = std::i32::MIN;
        let mut sum = 0;
        for num in nums.iter() {
            sum += num;
            max_sum = std::cmp::max(max_sum, sum);
            if sum < 0 {
                sum = 0;
            }
        }
        max_sum
    }
}
