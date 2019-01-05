impl Solution {
    /// 简单粗暴 (
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_unstable_by_key(|&n| n & 1);
        a
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let mut ret = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
        ret[..2].sort_unstable();
        ret[2..].sort_unstable();
        assert_eq!(ret, vec![2, 4, 1, 3]);
    }
}