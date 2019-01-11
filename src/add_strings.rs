impl Solution {
    pub fn add_strings<S: AsRef<str>>(num1: S, num2: S) -> String {
        let (mut num1, mut num2) = (num1.as_ref().bytes().rev(), num2.as_ref().bytes().rev());
        let mut ret = vec![];
        let mut carry = 0;
        loop {
            match (num1.next(), num2.next()) {
                (None, None) => break,
                (a, b) => {
                    let mut n = a.unwrap_or(b'0') + b.unwrap_or(b'0') - b'0' + carry;
                    carry = 0;
                    if n > b'9' {
                        n -= 10;
                        carry = 1;
                    }
                    ret.push(n as char);
                }
            }
        }
        if carry != 0 {
            ret.push('1');
        }
        ret.iter().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::add_strings("123", "456"), "579");
        assert_eq!(Solution::add_strings("999", "10000"), "10999");
        assert_eq!(Solution::add_strings("999", "1"), "1000");
    }
}
