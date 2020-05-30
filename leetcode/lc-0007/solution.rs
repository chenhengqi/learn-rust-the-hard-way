impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut lx: i64 = x.into();
        let negative = if lx < 0 {
            lx *= -1;
            true
        } else {
            false
        };

        let mut rx: i64 = 0;
        while lx != 0 {
            rx = rx * 10 + lx % 10;
            lx /= 10;
            if negative {
                if -rx < std::i32::MIN.into() {
                    return 0;
                }
            } else {
                if rx > std::i32::MAX.into() {
                    return 0;
                }
            }
        }
        if negative {
            return -rx as i32;
        }
        rx as i32
    }
}
