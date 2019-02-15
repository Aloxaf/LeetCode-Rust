impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
