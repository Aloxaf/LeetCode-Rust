impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        j.bytes().fold(0, |cnt, b1| cnt + s.bytes().filter(|&b2| b1 == b2).count()) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::num_jewels_in_stones("aA".to_owned(),"aAAbbbb".to_owned()), 3);
        assert_eq!(Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned()), 0);
    }
}