use std::mem;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut head) = curr {
            let next = mem::replace(&mut head.next, None);
            head.next = prev;
            prev = Some(head);
            curr = next;
        }

        prev
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse_list(linkedlist![1, 2, 3, 4, 5]), linkedlist![5, 4, 3, 2, 1]);
    }
}