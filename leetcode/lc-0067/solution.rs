impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a == "0" {
            return b;
        }
        if b == "0" {
            return a;
        }

        // 111 + 111 = 1011 => 4 digits
        let m = a.len();
        let n = b.len();
        let k = std::cmp::max(m, n) + 1;
        let mut result = vec![b'0'; k];

        let mut j = k - 1;
        let a = a.as_bytes();
        for i in (0..m).rev() {
            result[j] += (a[i] - b'0');
            j -= 1;
        }
        let mut j = k - 1;
        let b = b.as_bytes();
        for i in (0..n).rev() {
            result[j] += (b[i] - b'0');
            j -= 1;
        }
        for i in (1..k).rev() {
            let sum = result[i] - b'0';
            result[i] = (sum as i32 % 2) as u8 + b'0';
            result[i - 1] += (sum / 2) as u8;
        }

        for i in 0..k {
            if result[i] != b'0' {
                return String::from_utf8(result[i..].to_vec()).unwrap();
            }
        }
        String::from_utf8(result).unwrap()
    }
}
