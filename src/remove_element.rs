impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // 最佳实践: nums.remove_item(&val);
        // 可惜还没稳定
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut v = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut v, 3), 2);
        assert_eq!(&v[0..2], [2, 2]);

        let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut v, 2), 5);
        &v[0..5].sort();
        assert_eq!(&v[0..5], [0, 0, 1, 3, 4]);
    }
}