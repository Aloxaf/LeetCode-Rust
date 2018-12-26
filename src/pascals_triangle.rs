impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        let mut ret = vec![vec![1]];
        for i in 1..num_rows as usize {
            let mut tmp = vec![1];
            for j in 1..i {
                tmp.push(ret[i - 1][j - 1] + ret[i - 1][j]);
            }
            tmp.push(1);
            ret.push(tmp);
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
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );

        assert_eq!(Solution::generate(1), vec![vec![1],]);
        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
    }
}
