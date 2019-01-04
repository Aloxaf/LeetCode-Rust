use std::mem;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.is_some() {
            if cur.as_ref().unwrap().val == val {
                let mut tmp = mem::replace(cur, None);
                mem::swap(cur, &mut tmp.as_mut().unwrap().next);
                continue;
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod test {
    use crate::linkedlist;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_elements(linkedlist![1, 2, 3, 4, 5, 6], 6),
            linkedlist![1, 2, 3, 4, 5]
        );
    }
}
