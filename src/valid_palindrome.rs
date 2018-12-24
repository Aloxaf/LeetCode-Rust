impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.bytes().filter_map(|b| {
            if b.is_ascii_alphanumeric() { Some(b.to_ascii_lowercase()) } else { None }
        }).collect::<Vec<_>>();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(Solution::is_palindrome("".to_owned()), true);
    }
}