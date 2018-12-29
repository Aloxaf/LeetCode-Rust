/// 基本思路是去掉括号, 然后传给 calc 计算
impl Solution {
    #[inline]
    fn calc(nums: Vec<i32>, ops: Vec<u8>) -> i32 {
        let mut ret = nums[0];
        for (&op, &num) in ops.iter().zip(&nums[1..]) {
            match op {
                b'+' => ret += num,
                _ => ret -= num,
            }
        }
        ret
    }

    pub fn calculate(s: String) -> i32 {
        let mut positive = true;      // 记录当前外层括号前的符号
        let mut nums = vec![];
        let mut ops = vec![];
        let mut num = (false, 0);  // 数字的临时变量, 其中 .0 用于记录是否正在解析数字
        let mut prev = b'+';            // 记录最后一个运算符
        let mut sign = vec![];    // 记录括号前的符号
        for c in s.bytes().filter(|c| !c.is_ascii_whitespace()) {
            match c {
                b'0'..=b'9' => num = (true, num.1 * 10 + i32::from(c - b'0')),
                _ => {
                    if num.0 {
                        nums.push(num.1);
                        num = (false, 0);
                    }
                    match c {
                        // 根据情况选择是否变号
                        b'+' if positive => ops.push(b'+'),
                        b'-' if positive => ops.push(b'-'),
                        b'+' if !positive => ops.push(b'-'),
                        b'-' if !positive => ops.push(b'+'),
                        b'(' => {
                            // 遇到左括号, 将当前外层符号压栈, 并更新下一层的符号
                            sign.push(positive);
                            positive = prev == b'+';
                        },
                        b')' => positive = sign.pop().unwrap(), // 遇到右括号, 恢复外层的符号
                        _ => unreachable!(),
                    }
                }
            }
            prev = *ops.last().unwrap_or(&b'+');
        }
        if num.0 {
            nums.push(num.1);
        }
        Solution::calc(nums, ops)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("1-(1)+8".to_owned()), 8);
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("1-(1-1-(1-1)) ".to_owned()), 1);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("1 - 1 ".to_owned()), 0);
        assert_eq!(Solution::calculate("1 - (5) ".to_owned()), -4);
        assert_eq!(Solution::calculate("2 - (5-6) ".to_owned()), 3);
        assert_eq!(Solution::calculate("2-4-(8+2-6+(8+4-(1)+8-10))".to_owned()), -15);
    }
}
