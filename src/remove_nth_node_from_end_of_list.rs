use crate::ListNode;

impl Solution {
    // 啊！ 要被 borrow checker 逼疯了
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut slow = &dummy;
        let mut fast = &dummy;
        for _ in 0..n {
            fast = &fast.as_ref().unwrap().next;
        }
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }
        let a = slow as *const _ as *mut Option<Box<ListNode>>;
        let b = &slow.as_ref().unwrap().next as *const _ as *mut Option<Box<ListNode>>;
        unsafe { std::ptr::swap(a, b); }
        dummy.unwrap().next
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::{linkedlist, ListNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![1, 2, 3, 5]
        );
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1, 2, 3, 4, 5], 1),
            linkedlist![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1], 1),
            linkedlist![]
        );
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1, 2], 2),
            linkedlist![2]
        );
    }
}
