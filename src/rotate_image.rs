impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len {
            for j in i+1..len {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
            for j in 0..len / 2 {
                matrix[i].swap(j, len - j - 1);
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut v);
        assert_eq!(v, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }
}
