impl Solution {
    // 用了迭代器也好慢啊, 16ms
    // 难道是 String 的处理太慢了?
    pub fn int_to_roman(mut num: i32) -> String {
        let map: Vec<(usize, &str)> = vec![
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
            (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
        ];
        map.iter().map(|(k, v)| {
            let times = num as usize / *k;
            num %= *k as i32;
            v.repeat(times)
        }).collect::<Vec<_>>().join("")
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}