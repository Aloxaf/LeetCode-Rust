use std::collections::HashSet;
const MAP: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
    "-.--", "--..",
];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        words.iter().fold(HashSet::new(), |mut set, s| {
            set.insert(s.bytes().map(|b| MAP[(b - b'a') as usize]).collect::<Vec<_>>().join(""));
            set
        }).len() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::unique_morse_representations(
                ["gin", "zen", "gig", "msg"]
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
            ),
            2
        );
    }
}
