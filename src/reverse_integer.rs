// 这个地方为啥不设计成 fn reverse(x: i32) -> Option<i32> 呢
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret = (0i32, false);
        let mut x = x;

        while x != 0 {
            let n = x % 10;
            x /= 10;
            ret = ret.0.overflowing_mul(10);
            if ret.1 { return 0; }
            ret = ret.0.overflowing_add(n);
            if ret.1 { return 0; }
        }
        ret.0
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}