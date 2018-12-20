impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 最佳实践: nums.dedup();
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        1 + i as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut v = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(&v[0..2], [1, 2]);

        let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v), 5);
        assert_eq!(&v[0..5], [0, 1, 2, 3, 4]);
    }
}