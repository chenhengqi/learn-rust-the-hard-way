impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => {
                let mut result = vec![vec![1], vec![1, 1]];
                for i in 2..(num_rows as usize) {
                    let mut row = vec![1];
                    let n = result[i - 1].len() - 1;
                    for j in 0..n {
                        row.push(result[i - 1][j] + result[i - 1][j + 1]);
                    }
                    row.push(1);
                    result.push(row);
                }
                result
            },
        }
    }
}
