impl Solution {
    // 摩尔投票算法
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut major, mut count) = (nums[0], 1);
        for &n in &nums[1..] {
            if count == 0 {
                count = 1;
                major = n;
            } else if major == n {
                count += 1;
            } else {
                count -= 1;
            }
        }
        major
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
