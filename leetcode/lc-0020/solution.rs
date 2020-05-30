impl Solution {
    pub fn is_valid(s: String) -> bool {
        let is_pair = |l, r| {
            match (l, r) {
                ('(', ')') | ('[', ']') | ('{', '}') => true,
                _ => false,
            }
        };

        let mut t = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => t.push(c),
                _ => {
                    if let Some(l) = t.pop() {
                        if !is_pair(l, c) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        return t.is_empty();
    }
}
