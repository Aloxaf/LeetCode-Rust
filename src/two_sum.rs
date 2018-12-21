use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ret = vec![];
        for i in 0..nums.len() {
            if let Some(&idx) = map.get(&(target - nums[i])) {
                ret.extend([idx as i32, i as i32].iter());
                break;
            } else {
                map.insert(nums[i], i);
            }
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
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}
