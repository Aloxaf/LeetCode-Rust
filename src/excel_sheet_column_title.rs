impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut ret = String::new();
        while n != 0 {
            n -= 1;
            ret.push((b'A' + (n % 26) as u8)as char);
            n /= 26;
        }
        ret.chars().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
