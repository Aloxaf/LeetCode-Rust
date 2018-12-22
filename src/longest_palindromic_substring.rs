impl Solution {
    // 感觉写得很挫
    fn expand(s: &str, mut left: usize, mut right: usize) -> (&str, usize) {
        let bytes = s.as_bytes();
        let cnt = left.min(s.len() - right - 1);

        if bytes[left] != bytes[right] {
            return ("", 0);
        }
        for _ in 0..cnt {
            left  -= 1;
            right += 1;
            if bytes[left] != bytes[right] {
                return (&s[left+1..=right-1], right - left - 1);
            }
        }
        (&s[left..=right], right - left + 1)
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.len() > 1 {
            let mut longest = ("", 0);
            for i in 0..s.len() - 1 {
                let a1 = Solution::expand(&s, i, i);
                let a2 = Solution::expand(&s, i, i + 1);
                let ans = if a1.1 > a2.1 { a1 } else { a2 };
                if ans.1 > longest.1 {
                    longest = ans;
                }
            }
            return longest.0.into();
        }
        s
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_expand() {
        assert_eq!(Solution::expand("cbbd", 0, 0), ("c", 1));
        assert_eq!(Solution::expand("cbbd", 0, 1), ("", 0));
        assert_eq!(Solution::expand("cbbd", 1, 2), ("bb", 2));
        assert_eq!(Solution::expand("a", 0, 0), ("a", 1));
    }

    #[test]
    fn test() {
        // 这个地方没考虑多解的样例
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("a".to_owned()), "a");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
        assert_eq!(Solution::longest_palindrome("SQQSYYSQQS".to_owned()), "SQQSYYSQQS");
    }
}