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

    /// 妙啊妙啊
    /// https://leetcode.com/problems/valid-sudoku/discuss/15472/Short%2BSimple-Java-using-Strings
    /// 对于 i 行 j 列的数字 c, 将 (None, c, j) (i, c, None) (i / 3, c, j / 3) 插入一个 HashSet 中
    /// 若插入时发现键已存在, 则说明对应 列/行/九宫格 存在重复数字
    /// 不过竟然更慢了? 总感觉是 Rust Hash 算法的锅...
    pub fn is_valid_sudoku_hashset(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for (i, l) in board.iter().enumerate() {
            for (j, &m) in l.iter().enumerate() {
                if m != '.' {
                    if !seen.insert((None, m, Some(j)))
                        || !seen.insert((Some(i), m, None))
                        || !seen.insert((Some(i / 3), m, Some(j / 3)))
                    {
                        return false
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

    #[test]
    fn test_hashset() {
        assert_eq!(
            Solution::is_valid_sudoku_hashset(vec![
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
            Solution::is_valid_sudoku_hashset(vec![
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
