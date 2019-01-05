use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for email in &emails {
            let mut email = email.split('@');
            let name = email
                .next()
                .unwrap()
                .chars()
                .filter_map(|c| if c == '.' { None } else { Some(c) })
                .take_while(|&c| c != '+')
                .collect::<String>();
            set.insert((name, email.next().unwrap()));
        }
        set.len() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_string;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_unique_emails(vec_string![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]),
            2
        );
    }
}
