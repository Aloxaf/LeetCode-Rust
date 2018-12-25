#[derive(Debug, PartialOrd, PartialEq)]
enum Sym {
    Num(i32),
    Op(u8),
}

impl Solution {
    #[inline]
    fn calc(stack: Vec<Sym>) -> i32 {
        // println!("{:?}", &stack);
        let mut nums = vec![];
        for sym in stack {
            match sym {
                Sym::Num(num) => nums.push(num),
                Sym::Op(op) => {
                    let (n2, n1) = (nums.pop().unwrap(), nums.pop().unwrap());
                    match op {
                        b'+' => nums.push(n1 + n2),
                        _ => nums.push(n1 - n2),
                    }
                }
            }
        }
        nums.pop().unwrap()
    }

    // 本来也想试试单趟搞定, 然而写出来一堆bug, 怒而逆波兰
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut output = vec![];
        let mut i = 0;
        while i < s.len() {
            let c = s[i];
            match c {
                b'0'..=b'9' => {
                    let mut num = (c - b'0') as i32;
                    while s.get(i + 1).unwrap_or(&b' ').is_ascii_digit() {
                        i += 1;
                        num = num * 10 + (s[i] - b'0') as i32;
                    }
                    output.push(Sym::Num(num));
                },
                b'+' | b'-' => {
                    while let Some(op) = stack.pop() {
                        if op == Sym::Op(b'(') {
                            stack.push(op);
                            break;
                        }
                        output.push(op);
                    }
                    stack.push(Sym::Op(c))
                },
                b'(' => stack.push(Sym::Op(c)),
                b')' => {
                    while let Some(op) = stack.pop() {
                        if op == Sym::Op(b'(') {
                            break;
                        }
                        output.push(op);
                    }
                }
                _ => (),
            }
            i += 1;
        }
        stack.reverse();
        output.append(&mut stack);
        Solution::calc(output)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("1 - 1 ".to_owned()), 0);
        assert_eq!(Solution::calculate("1 - (5) ".to_owned()), -4);
        assert_eq!(Solution::calculate("2 - (5-6) ".to_owned()), 3);
        assert_eq!(Solution::calculate("2-4-(8+2-6+(8+4-(1)+8-10))".to_owned()), -15);
    }
}
