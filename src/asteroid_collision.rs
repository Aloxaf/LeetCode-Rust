impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut battlefield = vec![];
        for i in asteroids {
            if i > 0 {
                battlefield.push(i);
            } else {
                while let Some(&l) = battlefield.last() {
                    if l >= -i || l < 0 {
                        break;
                    }
                    battlefield.pop();
                }
                let last = battlefield.last();
                if last.is_none() || last.unwrap() < &0 {
                    battlefield.push(i);
                } else if last.is_some() && last.unwrap() == &-i {
                    battlefield.pop();
                }
            }
        }
        battlefield
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
        assert_eq!(Solution::asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
    }
}
