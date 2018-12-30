//! 这题目名字真长
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut ret = vec![-1, -1];

        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if nums[left] != target {
            return vec![-1, -1];
        } else {
            ret[0] = left as i32;
        }

        // 左边界不需要重置, 节省时间
        right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2 + 1;
            if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        ret[1] = right as i32;
        ret
    }

    pub fn search_range_builtin(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = nums.iter().position(|&n| n == target).map(|n| n as i32);
        let right = nums.iter().rposition(|&n| n == target).map(|n| n as i32);
        vec![left.unwrap_or(-1), right.unwrap_or(-1)]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn custom() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 6), vec![-1, -1]);
    }

    #[test]
    fn builtin() {
        assert_eq!(Solution::search_range_builtin(vec![5,7,7,8,8,10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range_builtin(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn custom(b: &mut Bencher) {
        b.iter(|| Solution::search_range((1..100).collect(), 50));
    }

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::search_range_builtin((1..100).collect(), 50));
    }
}
