use std::cmp::Ordering;

impl Solution {
    // (a + b).cmp(b + a)
    #[inline]
    fn wow_cmp(a: &str, b: &str) -> Ordering {
        let iter1 = a.bytes().chain(b.bytes());
        let iter2 = b.bytes().chain(a.bytes());
        for (i, j) in iter1.zip(iter2) {
            match i.cmp(&j) {
                Ordering::Equal => continue,
                cmp => return cmp,
            }
        }
        Ordering::Equal
    }

    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| Solution::wow_cmp(b, a));
        match nums.join("").trim_start_matches('0') {
            "" => "0".to_owned(),
            s => s.to_owned(),
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::cmp::Ordering;

    #[test]
    fn test_wow_cmp() {
        assert_eq!(Solution::wow_cmp("3", "30"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("3", "34"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("34", "30"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("10", "2"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("12", "128"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("128", "12"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("121", "12"), Ordering::Less);
    }

    #[test]
    fn test() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
        assert_eq!(Solution::largest_number(vec![128, 12]), "12812");
        assert_eq!(Solution::largest_number(vec![0, 10]), "100");
        assert_eq!(Solution::largest_number(vec![0, 0]), "0");
    }
}
