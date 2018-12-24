impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        *digits.last_mut().unwrap() += 1;
        let mut ret = vec![];
        let mut carry = 0;
        for &i in digits.iter().rev() {
            let mut digit = i + carry;
            carry = 0;
            if digit >= 10 {
                digit -= 10;
                carry = 1;
            }
            ret.push(digit);
        }
        if carry != 0 {
            ret.push(1);
        }
        ret.reverse();
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}