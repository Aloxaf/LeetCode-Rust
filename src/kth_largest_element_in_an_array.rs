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

    // 快速选择
    pub fn find_kth_largest_qselect(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        let k = nums.len() as i32 - k;
        while left < right {
            let idx = Self::partition(&mut nums, left, right);
            if idx < k  {
                left = idx + 1;
            } else if idx > k {
                right = idx - 1;
            } else {
                break;
            }
        }
        nums[k as usize]
    }

    #[inline]
    fn partition(nums: &mut Vec<i32>, left: i32, right: i32) -> i32 {
        // pivot 随便取了...
        let pivot = nums[left as usize];
        let (mut i, mut j) = (left - 1, right + 1);
        loop {
            // do while (
            while {i += 1; nums[i as usize] < pivot} {}
            while {j -= 1; nums[j as usize] > pivot} {}
            if i >= j {
                return j;
            }
            nums.swap(i as usize, j as usize);
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn basic_test(f: fn(Vec<i32>, i32) -> i32) {
        assert_eq!(f(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(f(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn sort() {
        basic_test(Solution::find_kth_largest_sort);
    }

    #[test]
    fn pq() {
        basic_test(Solution::find_kth_largest_pq);
    }

    #[test]
    fn qselect() {
        basic_test(Solution::find_kth_largest_qselect);
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

    #[bench]
    fn qselect(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut v = (1..100).collect::<Vec<_>>();
        v.shuffle(&mut rng);
        b.iter(|| Solution::find_kth_largest_qselect(v.clone(), 10));
    }
}
