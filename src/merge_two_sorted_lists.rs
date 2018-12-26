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
            let (x, y) = (l1.map(|n|n.val), l2.map(|n|n.val));
            let (x, y) = (x.unwrap_or(std::i32::MAX), y.unwrap_or(std::i32::MAX));

            let n = if x < y {
                l1 = l1.map(|n|n.next.as_ref()).unwrap_or(None);
                x
            } else {
                l2 = l2.map(|n|n.next.as_ref()).unwrap_or(None);
                y
            };
            ref_head.next = Some(Box::new(ListNode::new(n)));
            ref_head = ref_head.next.as_mut().unwrap();
        }

        head.next
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use crate::linkedlist;
    use super::Solution;

    #[test]
    fn test() {
        let l1 = linkedlist![1, 2, 4];
        let l2 = linkedlist![1, 3, 4];
        assert_eq!(Solution::merge_two_lists(l1, l2), linkedlist![1, 1, 2, 3, 4, 4]);
    }
}