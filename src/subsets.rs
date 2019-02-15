impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]; 1 << nums.len()];
        for i in 0..nums.len() {
            for j in 0..1 << nums.len() {
                if (j >> i) & 1 == 1 {
                    ret[j].push(nums[i]);
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::assert_eq_sorted;

    #[test]
    fn test() {
        assert_eq_sorted!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![3],
                vec![1],
                vec![2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2],
                vec![]
            ]
        );
    }
}
