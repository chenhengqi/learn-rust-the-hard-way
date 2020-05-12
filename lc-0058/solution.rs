impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut in_word = false;
        let mut len = 0;
        for ch in s.chars().rev() {
            match ch {
                ' ' => {
                    if in_word {
                        break;
                    }
                },
                _ => {
                    in_word = true;
                    len += 1;
                }
            }
        }
        len
    }
}
