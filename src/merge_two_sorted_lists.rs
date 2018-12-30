use crate::ListNode;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut ref_head = &mut head;

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        while l1.is_some() || l2.is_some() {
            let (x, y) = (l1.map(|n| n.val), l2.map(|n| n.val));

            let mut l1_next = |x: i32| {
                l1 = l1.map(|n| n.next.as_ref()).unwrap_or(None);
                x
            };
            let mut l2_next = |y: i32| {
                l2 = l2.map(|n| n.next.as_ref()).unwrap_or(None);
                y
            };
            let n = match (x, y) {
                (Some(x), Some(y)) => if x < y { l1_next(x) } else { l2_next(y) },
                (Some(x), None) => l1_next(x),
                (None, Some(y)) => l2_next(y),
                _ => unreachable!()
            };

            ref_head.next = Some(Box::new(ListNode::new(n)));
            ref_head = ref_head.next.as_mut().unwrap();
        }

        head.next
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