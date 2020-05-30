impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut left = 0;
        let mut right = x as i64;
        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;
            if square == x as i64 {
                return mid as i32;
            } else if square > x as i64 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        if left * left > x as i64 { (left - 1) as i32 } else { left as i32 }
    }
}
