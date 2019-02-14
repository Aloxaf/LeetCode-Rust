impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
        let (mut m1, mut m2) = (0, 1);
        let (mut c1, mut c2) = (0, 0);
        for &n in &nums {
            if m1 == n {
                c1 += 1;
            } else if m2 == n {
                c2 += 1;
            } else if c1 == 0 {
                m1 = n;
                c1 = 1;
            } else if c2 == 0 {
                m2 = n;
                c2 = 1;
            } else {
                c1 -= 1;
                c2 -= 1;
            }
        }
        [m1, m2].iter().filter(|&&n| {
            nums.iter().filter(|&&x| x == n).count() * 3 > nums.len()
        }).cloned().collect()
    }
}

pub struct Solution;


#[cfg(test)]
mod test {
    use super::Solution;
    use crate::assert_eq_sorted;

    #[test]
    fn test() {
        assert_eq_sorted!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq_sorted!(Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]), vec![1, 2]);
        assert_eq_sorted!(Solution::majority_element(vec![]), vec![]);
        assert_eq_sorted!(Solution::majority_element(vec![2, 2]), vec![2]);
        assert_eq_sorted!(Solution::majority_element(vec![6, 6, 6, 7, 7]), vec![6, 7]);
    }
}
