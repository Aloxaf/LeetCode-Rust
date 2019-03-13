impl Solution {
    pub fn add_digits_recursive(num: i32) -> i32 {
        let ret = num.to_string().bytes().fold(0, |acc, c| acc + (c - b'0') as i32);
        if ret >= 10 {
            Self::add_digits_recursive(ret)
        } else {
            ret
        }
    }

    // https://en.wikipedia.org/wiki/Digital_root#Congruence_formula
    pub fn add_digits(num: i32) -> i32 {
        1 + (num - 1) % 9
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(Solution::add_digits_recursive(38), 2);
    }
}
