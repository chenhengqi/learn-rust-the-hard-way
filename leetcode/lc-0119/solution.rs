use std::mem::swap;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => vec![1],
            1 => vec![1, 1],
            2 => vec![1, 2, 1],
            _ => {
                let mut current: Vec<i32> = vec![1, 2, 1];
                let mut next: Vec<i32> = vec![];
                next.reserve(40);
                for i in 2..(row_index as usize) {
                    next.clear();
                    next.push(1);
                    for j in 0..i {
                        next.push(current[j] + current[j + 1]);
                    }
                    next.push(1);
                    swap(&mut current, &mut next);
                }
                current
            },
        }
    }
}
