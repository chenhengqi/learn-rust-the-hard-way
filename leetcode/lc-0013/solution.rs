impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let to_int = |c| {
            match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => panic!("unreachable"),
            }
        };
        let mut num = 0;
        let mut prev = 0;
        let n = s.len();
        for i in 0..n {
            let curr = to_int(s.as_bytes()[i]);
            if (curr > prev) {
                num += curr - 2 * prev;
            } else {
                num += curr;
            }
            prev = curr;
        }
        return num;
    }
}
