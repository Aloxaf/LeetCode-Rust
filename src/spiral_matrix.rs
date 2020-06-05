impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let (row, column) = (matrix.len(), matrix[0].len());
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut steps = [column, row - 1];
        let mut ret = vec![];

        let mut idir = 0;
        let (mut x, mut y) = (0, -1);
        while steps[idir % 2] != 0 {
            for _ in 0..steps[idir % 2] {
                x = (x as isize + dirs[idir].0) as usize;
                y = y + dirs[idir].1;
                ret.push(matrix[x][y as usize]);
            }
            steps[idir % 2] -= 1;
            idir = (idir + 1) % 4;
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }
}
