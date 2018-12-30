use crate::ListNode;

use std::mem;

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        let (mut l1, mut l2) = (&mut l1, &mut l2);
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            let (v1, v2) = (n1.val, n2.val);
            let x = if v1 < v2 {
                l1 = &mut l1.as_mut().unwrap().next;
                v1
            } else {
                l2 = &mut l2.as_mut().unwrap().next;
                v2
            };
            // 这个地方我本来试图用 mem::swap 避免新建节点, 然而未能成功战胜 borror checker,
            // 如果有谁实现了还望不啬赐教
            cur.next = Some(Box::new(ListNode::new(x)));
            cur = cur.next.as_mut().unwrap();
        }

        // 如果 l1 仍然有剩余节点, 直接将 cur.next 指向 l1, 反之亦然
        if l1.is_some() {
            mem::swap(&mut cur.next, l1);
        } else {
            mem::swap(&mut cur.next, l2);
        }

        dummy.next
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::linkedlist;
    use super::Solution;

    #[test]
    fn test() {
        let l1 = linkedlist![1, 2, 4];
        let l2 = linkedlist![1, 3, 4];
        assert_eq!(Solution::merge_two_lists(l1, l2), linkedlist![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test2() {
        let l1 = linkedlist![1, 2, 2147483647];
        let l2 = linkedlist![1, 2, 4];
        assert_eq!(Solution::merge_two_lists(l1, l2), linkedlist![1, 1, 2, 2, 4, 2147483647]);
    }
}