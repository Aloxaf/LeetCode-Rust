impl Solution {
    pub fn find_kth_largest_sort(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }

    pub fn find_kth_largest_pq(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut queue = BinaryHeap::from(nums);
        for _ in 0..(k - 1) {
            queue.pop();
        }
        *queue.peek().unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn basic_test(f: fn(Vec<i32>, i32) -> i32) {
        assert_eq!(f(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(f(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
        assert_eq!(f(vec![7, 6, 5, 4, 3, 2, 1], 2), 6);
    }

    #[test]
    fn sort() {
        basic_test(Solution::find_kth_largest_sort);
    }

    #[test]
    fn pq() {
        basic_test(Solution::find_kth_largest_pq);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[bench]
    fn sort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut v = (1..100).collect::<Vec<_>>();
        v.shuffle(&mut rng);
        b.iter(|| Solution::find_kth_largest_sort(v.clone(), 10));
    }

    #[bench]
    fn pq(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut v = (1..100).collect::<Vec<_>>();
        v.shuffle(&mut rng);
        b.iter(|| Solution::find_kth_largest_pq(v.clone(), 10));
    }
}
