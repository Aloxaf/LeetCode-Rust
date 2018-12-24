impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut b1, mut b2) = (a.bytes().rev(), b.bytes().rev());
        let mut ret = vec![];
        let mut carry = 0;
        for _ in 0..a.len().max(b.len()) {
            let mut bit = b1.next().unwrap_or(b'0') + b2.next().unwrap_or(b'0') - b'0' * 2 + carry;
            carry = 0;
            if bit >= 2 {
                bit -= 2;
                carry = 1;
            }
            ret.push((bit + b'0') as char);
        }
        if carry == 1 {
            ret.push('1');
        }
        ret.reverse();
        ret.iter().collect::<String>()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::add_binary("11".to_owned(), "1".to_owned()), "100");
        assert_eq!(Solution::add_binary("1010".to_owned(), "1011".to_owned()), "10101");
    }
}
