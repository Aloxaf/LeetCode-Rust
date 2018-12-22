impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // 直接 split_whitespace 好像太作弊了点
        // s.trim().split_whitespace().last().map(|s| s.len()).unwrap_or(0) as i32
        s.bytes().rev().skip_while(|&c| c == b' ').take_while(|&c| c != b' ' ).count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("a ".to_owned()), 1);
    }
}