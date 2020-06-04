impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut ret = String::new();
        let (mut x, mut y) = (0, 0);
        for c in target.bytes() {
            let row = (c - b'a') / 5;
            let column = (c - b'a') % 5;
            while x != row || y != column {
                while x < row && !(y != 0 && x == 4) {
                    x += 1;
                    ret.push('D');
                }
                while x > row {
                    x -= 1;
                    ret.push('U');
                }
                while y < column && !(x == 5 && y == 0) {
                    y += 1;
                    ret.push('R');
                }
                while y > column {
                    y -= 1;
                    ret.push('L');
                }
            }
            ret.push('!');
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            &Solution::alphabet_board_path("leet".into()),
            "DDR!UURRR!!DDD!"
        );
        assert_eq!(
            &Solution::alphabet_board_path("code".into()),
            "RR!DDRR!UUL!R!"
        );
        assert_eq!(
            &Solution::alphabet_board_path("zdz".into()),
            "DDDDD!UUUUURRR!DDDDLLLD!"
        );
    }
}
