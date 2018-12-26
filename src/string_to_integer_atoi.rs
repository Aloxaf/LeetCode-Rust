impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut iter = str.chars().skip_while(|&c| c.is_ascii_whitespace());
        match iter.next().unwrap_or('x') {
            c @ '+' | c @ '-' | c @ '0'..='9' => {
                let mut s = String::new();
                let mut ret = (0i32, false);
                let positive = match c {
                    '-' => false,
                    '+' => true,
                    _ => {s.push(c); true},
                };
                s.push_str(&iter.take_while(|c| c.is_ascii_digit()).collect::<String>());

                for c in s.bytes() {
                    let n = c - b'0';
                    ret = ret.0.overflowing_mul(10);
                    if ret.1 { break; }
                    ret = ret.0.overflowing_add(i32::from(n));
                    if ret.1 { break; }

                }

                if ret.1 {
                    if positive { std::i32::MAX } else { std::i32::MIN }
                } else if positive {
                    ret.0
                } else {
                    -ret.0
                }
            },
            _ => 0
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    
    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi("42".into()), 42);
        assert_eq!(Solution::my_atoi("   -42".into()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".into()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".into()), -2147483648);
    }
}
