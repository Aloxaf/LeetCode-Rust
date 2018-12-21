impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(min_length) = strs.iter().map(|s| s.len()).min() {
            let strs = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
            for i in 0..min_length {
                for j in 1..strs.len() {
                    if strs[0][i] != strs[j][i] {
                        return std::str::from_utf8(&strs[0][0..i]).unwrap().into()
                    }
                }
            }
            std::str::from_utf8(&strs[0][0..min_length]).unwrap().into()
        } else {
            "".into()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            &Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl"
        );
        assert_eq!(
            &Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            ""
        );
        assert_eq!(
            &Solution::longest_common_prefix(vec![]),
            ""
        );
        assert_eq!(
            &Solution::longest_common_prefix(vec!["".into(), "b".into()]),
            ""
        );
    }
}
