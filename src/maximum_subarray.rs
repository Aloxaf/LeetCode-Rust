impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut sum, mut max) = (nums[0], nums[0]);
        for i in &nums[1..] {
            if sum < 0 {
                sum = 0;
            }
            sum += i;
            max = max.max(sum);
        }
        max
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    
    #[test]
    fn test() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(Solution::max_sub_array(vec![-2, 1]), 1);
        assert_eq!(Solution::max_sub_array(vec![-2]), -2);
    }
}