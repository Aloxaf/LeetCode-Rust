impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        for &i in [2, 3, 5].iter() {
            while num != 0 && num % i == 0 {
                num /= i;
            }
        }
        num == 1
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(8), true);
        assert_eq!(Solution::is_ugly(14), false);
        assert_eq!(Solution::is_ugly(1), true);
    }
}
