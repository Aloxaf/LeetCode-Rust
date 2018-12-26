impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums = vec![];
        for s in &tokens {
            let s = s as &str;
            match s {
                "+" | "-" | "*" | "/" => {
                    let (y, x) = (nums.pop().unwrap(), nums.pop().unwrap());
                    let ans = match s {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        _ => unreachable!(),
                    };
                    nums.push(ans);
                },
                _ => nums.push(s.parse::<i32>().unwrap()),
            }
        }
        nums[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::eval_rpn(
                ["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            9
        );
        assert_eq!(
            Solution::eval_rpn(
                ["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            6
        );
        assert_eq!(
            Solution::eval_rpn(
                ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            22
        );
    }
}
