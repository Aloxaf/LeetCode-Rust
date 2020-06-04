impl Solution {
    /// assume that nums = [a, b, c, d, e]
    /// pass1: nums = [1, a, ab, abc, abcd]
    /// pass2: nums = [bcde, cde, de, e, 1]
    /// pass1 * pass2: nums = [bcde, acde, abde, abce, abcd]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut p = 1;
        for &i in nums.iter() {
            ret.push(p);
            p *= i;
        }
        p = 1;
        for (i, &j) in ret.iter_mut().zip(nums.iter()).rev() {
            *i = *i * p;
            p *= j;
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
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}