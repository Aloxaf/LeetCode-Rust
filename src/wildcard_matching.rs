impl Solution {
    // 使用泛型以同时接受 String, &str. 测试的时候可以少写一点代码!
    // Yu 的神奇算法, 抓住了 '*' 再多都只需要考虑一个这一点
    // http://yucoding.blogspot.com/2013/02/leetcode-question-123-wildcard-matching.html
    pub fn is_match<S: AsRef<str>>(s: S, p: S) -> bool {
        let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
        let (mut ss, mut pp, mut m, mut star) = (0, 0, 0, None);
        while ss < s.len() {
            // pattern 为 '?' 或者 str == pattern, 继续比较下一个
            if p.get(pp) == Some(&b'?') || s.get(ss) == p.get(pp) {
                ss += 1;
                pp += 1;
            // 否则, 当 pattern 为 '*' 时
            } else if p.get(pp) == Some(&b'*') {
                // 保存这时 pattern 的位置
                star = Some(pp);
                // 保存此时(已匹配) str 的位置
                m = ss;
                // str不变, pattern 继续下一个 (相当于 '*' 匹配 0 个字符的情况)
                pp += 1;
            // (这个时候遇到了不匹配的情况)
            // 如果 star 存在记录, 也就是前面有 * 字符
            } else if star.is_some() {
                // pattern 跳到那个 '*' 的下一个
                pp = star.unwrap() + 1;
                // str 跳到一开始匹配的地方的下一个
                // 相当于令 '*' 多匹配一个字符
                m += 1;
                ss = m;
            // 否则, 匹配失败
            } else {
                return false;
            }
        }
        while p.get(pp) == Some(&b'*') {
            pp += 1;
        }
        pp == p.len()
    }

    /// 超时的简单粗暴的垃圾算法
    fn _is_match(s: &[u8], p: &[u8]) -> bool {
        let (mut i, mut j) = (0, 0);
        loop {
            match (s.get(i), p.get(j)) {
                (Some(_), Some(b'?')) => (),
                (Some(_), Some(b'*')) => {
                    for k in i..=s.len() {
                        if Self::_is_match(&s[k..], &p[j+1..]) {
                            return true;
                        }
                    }
                },
                (Some(a), Some(b)) => if a != b { return false },
                (None, Some(b)) => if b != &b'*' { return false },
                (Some(_), None) => return false,
                (None, None) => return true,
            }
            i += 1;
            j += 1;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_normal() {
        assert_eq!(Solution::is_match("aa", "a"), false);
    }

    #[test]
    fn test_asterisk() {
        assert_eq!(Solution::is_match("aa", "*"), true);
        assert_eq!(Solution::is_match("adceb", "*a*b"), true);
        assert_eq!(Solution::is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab", "b*b*ab**ba*b**b***bba"), false);
    }

    #[test]
    fn test_question_mark() {
        assert_eq!(Solution::is_match("cb", "?a"), false);
    }

    #[test]
    fn test_all() {
        assert_eq!(Solution::is_match("acdcb", "a*c?b"), false);
    }
}
