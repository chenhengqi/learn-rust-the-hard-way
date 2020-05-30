impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        // if we reach here, then `right < left`
        if left >= nums.len() as i32 {
            return left;
        }
        if right < 0 {
            return 0;
        }
        if target <= right {
            return right;
        }
        left
    }
}
