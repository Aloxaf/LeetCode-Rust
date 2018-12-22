impl Solution {
    pub fn count_and_say(n: i32) -> String {
        match n {
            1 => "1".to_owned(),
            _ => {
                let s = Solution::count_and_say(n - 1);
                let s = s.as_bytes();
                let mut ret = String::new();
                let (mut prev, mut cnt) = (s[0], 1);
                for &c in &s[1..] {
                    if c != prev {
                        ret.push_str(&cnt.to_string());
                        ret.push(prev as char);
                        cnt = 0;
                    }
                    prev = c;
                    cnt += 1;
                }
                if cnt != 0 {
                    ret.push_str(&cnt.to_string());
                    ret.push(prev as char);
                }
                ret
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}