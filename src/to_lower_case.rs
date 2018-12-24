impl Solution {
    pub fn to_lower_case(str: String) -> String {
        str.bytes().map(|c| if c >= b'A' && c <= b'Z' { c ^ b' ' } else { c } as char).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::to_lower_case("Hello".to_owned()), "hello");
        assert_eq!(Solution::to_lower_case("here".to_owned()), "here");
        assert_eq!(Solution::to_lower_case("LOVELY".to_owned()), "lovely");
    }
}
