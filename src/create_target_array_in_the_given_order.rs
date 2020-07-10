impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(index.len());
        for (i, j) in nums.into_iter().zip(index.into_iter()) {
            ret.insert(j as usize, i);
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
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
    }
}
