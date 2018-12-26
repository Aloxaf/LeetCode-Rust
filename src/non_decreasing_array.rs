impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut possible = None;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                if possible.is_some() {
                    return false;
                }
                possible = Some(i);
            }
        }
        match possible {
            None => true,
            Some(i) => i == 0 || i == nums.len() - 2 || nums[i - 1] <= nums[i + 1] || nums[i] <= nums[i + 2],
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        assert_eq!(Solution::check_possibility(vec![-1, 4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![1, 5, 0, 4]), false);
        assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
        assert_eq!(Solution::check_possibility(vec![2, 3, 4, 2, 4]), true);
    }
}
