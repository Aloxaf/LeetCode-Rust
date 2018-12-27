use crate::ListNode;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(now) = p {
            while let Some(next) = now.next.as_mut() {
                if now.val != next.val {
                    break;
                }
                // 本来又上了 unsafe, 在 tg 群里问了下 @Xidorn Quan 大佬, 提示用 std::mem::replace, 妙啊!
                let next_next = std::mem::replace(&mut next.next, None);
                std::mem::replace(&mut now.next, next_next);
            }
            p = &mut now.next;
        }
        head
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2]),
            linkedlist![1, 2]
        );
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3]),
            linkedlist![1, 2, 3]
        );
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 1]),
            linkedlist![1]
        );
    }
}
