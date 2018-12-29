/// 逆波兰, 不用考虑括号真是简单粗暴
impl Solution {
    #[inline]
    fn level(op: u8) -> u8 {
        match op {
            b'+' | b'-' => 1,
            b'*' | b'/' => 2,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn calc(nums: &mut Vec<i32>, op: u8) {
        let (n2, n1) = (nums.pop().unwrap(), nums.pop().unwrap());
        let ret = match op {
            b'*' => n1 * n2,
            b'/' => n1 / n2,
            b'+' => n1 + n2,
            b'-' => n1 - n2,
            _ => unreachable!()
        };
        nums.push(ret);
    }

    pub fn calculate(s: String) -> i32 {
        let mut nums = vec![];
        let mut ops = vec![];
        let mut num = 0;
        for b in s.bytes().filter(|b| !b.is_ascii_whitespace()) {
            match b {
                b'0'..=b'9' => num = num * 10 + (b - b'0') as i32,
                _ => {
                    nums.push(num);
                    num = 0;
                    while let Some(p) = ops.pop() {
                        if Self::level(b) <= Self::level(p) {
                            Self::calc(&mut nums, p);
                        } else {
                            ops.push(p);
                            break;
                        }
                    }
                    ops.push(b);
                }
            }
        }
        nums.push(num);
        while let Some(p) = ops.pop() {
            Self::calc(&mut nums, p);
        }
        nums[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("3+2*2".to_owned()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".to_owned()), 1);
        assert_eq!(Solution::calculate(" 3+5/2 ".to_owned()), 5);
    }
}