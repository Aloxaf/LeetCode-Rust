impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|n|n as i32).unwrap_or(-1)
    }
}

impl Solution {
    // 发现暴力也能 0ms 过, 还以为要 KMP.
    // 想想也是....毕竟是简单题
    pub fn str_str_bf(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1; //错误码差评!
        }
        if needle.is_empty() {
            return 0;
        }
        for i in 0..=(haystack.len().saturating_sub(needle.len())) {
            if &haystack[i..i+needle.len()] == &needle {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("aaaaa".into(), "bba".into()), -1);
        assert_eq!(Solution::str_str("hello".into(), "".into()), 0);
        assert_eq!(Solution::str_str("a".into(), "a".into()), 0);
        assert_eq!(Solution::str_str("mississippi".into(), "pi".into()), 9);
        assert_eq!(Solution::str_str("".into(), "a".into()), -1);
    }
}