impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let n = strs.len();
        if n == 1 {
            return strs[0].clone();
        }

        let prefix = strs[0].clone();
        for i in 0..prefix.len() {
            for j in 1..n {
                if strs[j].len() <= i {
                    return prefix.chars().take(i).collect();
                }
                if strs[j].as_bytes()[i] != prefix.as_bytes()[i] {
                    return prefix.chars().take(i).collect();
                }
            }
        }
        return prefix;
    }
}
