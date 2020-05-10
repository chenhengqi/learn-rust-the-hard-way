impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let say = |s: String| {
            let mut r = String::new();
            let mut c = s.as_bytes()[0];
            let mut n = 1;
            for i in 1..s.len() {
                let ch = s.as_bytes()[i];
                if ch == c {
                    n += 1;
                } else {
                    r += &n.to_string();
                    r.push(c as char);
                    c = ch;
                    n = 1;
                }
            }
            r.push_str(&n.to_string());
            r.push(c.into());
            r
        };

        let mut s = "1".to_string();
        for i in 1..n {
            s = say(s);
        }
        s
    }
}
