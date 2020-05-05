impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/guard.html
        match x {
            0 => true,
            n if n < 0 || n % 10 == 0 => false,
            _ => {
                let s = x.to_string();
                let mut l = 0;
                let mut r = s.len() - 1;
                while l < r {
                    if s.as_bytes()[l] == s.as_bytes()[r] {
                        l += 1;
                        r -= 1;
                    } else {
                        return false;
                    }
                }
                true
            },
        }
    }
}
