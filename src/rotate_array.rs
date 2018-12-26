impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k as usize % nums.len() {
            let item = nums.pop().unwrap();
            nums.insert(0, item);
        }
    }

    // 不符合要求的代码 (不过快一点
    pub fn rotate_drain_append(nums: &mut Vec<i32>, k: i32) {
        let mut v = nums.drain(..nums.len() - k as usize % nums.len()).collect::<Vec<_>>();
        nums.append(&mut v);
    }

    // 符合要求, 但是跟 cheat 一样 (
    pub fn rotate_builtin(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        &nums.rotate_right(k as usize % len);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
        Solution::rotate(&mut v, 8);
        assert_eq!(v, vec![4, 5, 6, 7, 1, 2, 3]);
    }

    #[test]
    fn test_drain_append() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate_drain_append(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
        Solution::rotate_drain_append(&mut v, 8);
        assert_eq!(v, vec![4, 5, 6, 7, 1, 2, 3]);
    }

    #[test]
    fn test_builtin() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate_builtin(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
        Solution::rotate_builtin(&mut v, 8);
        assert_eq!(v, vec![4, 5, 6, 7, 1, 2, 3]);
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn pop_insert(b: &mut Bencher) {
        b.iter(|| {
            let mut v = vec![1; 1000];
            Solution::rotate(&mut v, 300);
        })
    }

    // 数据量大的时候显出优势
    #[bench]
    fn drain_append(b: &mut Bencher) {
        b.iter(|| {
            let mut v = vec![1; 1000];
            Solution::rotate_drain_append(&mut v, 300);
        })
    }

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| {
            let mut v = vec![1; 1000];
            Solution::rotate_builtin(&mut v, 300);
        })
    }
}