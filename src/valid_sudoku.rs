impl Solution {
    // 朴实蠢蠢的方法
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 行
        for l in &board {
            let mut map = [false; 9];
            for &i in l {
                if i == '.' { continue; }
                let c = i as usize - b'1' as usize;
                if map[c] { return false; }
                map[c] = true;
            }
        }
        // 列
        for i in 0..board[0].len() {
            let mut map = [false; 9];
            for l in &board {
                if l[i] == '.' { continue; }
                let c = l[i] as usize - b'1' as usize;
                if map[c] { return false; }
                map[c] = true;
            }
        }
        // 九宫格
        for i in &[0usize, 3, 6] {
            for j in &[0usize, 3, 6] {
                let mut map = [false; 9];
                for a in 0..3 {
                    for b in 0..3 {
                        let c = board[i + a][j + b] as u8;
                        if c == b'.' { continue; }
                        if map[(c - b'1') as usize] { return false; }
                        map[(c - b'1') as usize] = true;
                    }
                }
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );

        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
