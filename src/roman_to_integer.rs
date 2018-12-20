impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn c2i(c: u8) -> i32 {
            match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => unreachable!(),
            }
        }
        let s = s.as_bytes();
        let mut ret = 0;
        for (i, c) in s.iter().enumerate() {
            let n = c2i(*c);
            if i == s.len() - 1 || n >= c2i(s[i+1])  {
                ret += n;
            } else {
                ret -= n;
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("IV".into()), 4);
        assert_eq!(Solution::roman_to_int("IX".into()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}