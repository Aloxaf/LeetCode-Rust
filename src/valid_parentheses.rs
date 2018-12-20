impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn pair(c: u8) -> u8 {
            match c {
                b'(' => b')',
                b'[' => b']',
                _ => b'}',
            }
        }
        let mut stack = vec![];
        for c in s.bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(pair(c)),
                _ => {
                    let pop = stack.pop().unwrap_or(0);
                    if pop != c {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid("([)]".into()), false);
        assert_eq!(Solution::is_valid("{[]}".into()), true);
    }
}