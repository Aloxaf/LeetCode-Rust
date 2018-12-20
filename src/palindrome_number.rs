impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut ret = (0i32, false);
        let mut y = x;

        if x < 0 {
            false
        } else {
            while y != 0 {
                let n = y % 10;
                y /= 10;
                ret = ret.0.overflowing_mul(10);
                if ret.1 { return false; }
                ret = ret.0.overflowing_add(n);
                if ret.1 { return false; }
            }
            x == ret.0
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}