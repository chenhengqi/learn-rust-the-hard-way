impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.clone();
        let mut carry = 1;
        let n = digits.len();
        for i in (0..n).rev() {
            digits[i] += carry;
            carry = digits[i] / 10;
            digits[i] %= 10;
        }
        if carry != 0 {
            digits.insert(0, 1);
        }
        digits
    }
}
