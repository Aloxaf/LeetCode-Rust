use crate::ListNode;

use std::mem;

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        let (l1, l2) = (&mut l1, &mut l2);
        // 想试试不新建节点来完成
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            let (v1, v2) = (n1.val, n2.val);
            if v1 < v2 {
                mem::swap(&mut cur.next, l1);
                mem::swap(&mut cur.next.as_mut().unwrap().next, l1);
                // 这个地方我原本采用是下面这种写法, 然而被 borrow checker 无情驳回
                // 请教了 @iovxw 大佬才得知
                // l1 是对 cur.next.next 的可变借用, 循环末尾 cur 会被更新为 cur.next 的可变借用
                // 由于这两个变量都会被使用, 于是就冲突了 (原先在 NLL 的保护之下我一直以为这是合法的...
                // l1 = &mut cur.next.as_mut().unwrap().next;
            } else {
                mem::swap(&mut cur.next, l2);
                mem::swap(&mut cur.next.as_mut().unwrap().next, l2);
            }
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