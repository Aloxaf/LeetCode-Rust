impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m - 1, n - 1);
        for i in (0..nums1.len()).rev() {
            let a = nums1.get(m as usize).unwrap_or(&std::i32::MIN);
            let b = nums2.get(n as usize).unwrap_or(&std::i32::MIN);
            if a > b {
                nums1[i] = *a;
                m -= 1;
            } else {
                nums1[i] = *b;
                n -= 1;
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut v1 = vec![1, 2, 3, 0, 0, 0];
        let mut v2 = vec![2, 5, 6];
        Solution::merge(&mut v1, 3, &mut v2, 3);
        assert_eq!(v1, vec![1, 2, 2, 3, 5, 6]);

        let mut v1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let mut v2 = vec![1, 2, 2];
        Solution::merge(&mut v1, 6, &mut v2, 3);
        assert_eq!(v1, vec![-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
