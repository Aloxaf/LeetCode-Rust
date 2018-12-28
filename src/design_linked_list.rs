use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

// 题目已经给过了单链表, 来个双的
#[derive(Debug)]
pub struct MyLinkedList {
    len: Cell<i32>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    pub val: i32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            prev: None,
            next: None,
        }
    }
}

/** You can modify the type of `self` for your need. */
impl MyLinkedList {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        // MyLinkedList 的方法全是 &self, 没办法修改 head, tail 的值, 干脆两端都加个虚拟节点算了...
        let ret = MyLinkedList {
            len: Cell::new(0),
            head: Some(Rc::new(RefCell::new(Node::new(0)))),
            tail: Some(Rc::new(RefCell::new(Node::new(0)))),
        };
        ret.head.as_ref().unwrap().borrow_mut().next = ret.tail.clone();
        ret.tail.as_ref().unwrap().borrow_mut().prev = Some(Rc::downgrade(ret.head.as_ref().unwrap()));
        ret
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    pub fn get(&self, index: i32) -> i32 {
        if index >= self.len.get() || index < 0 {
            return -1;
        }
        self.locate(index).borrow().next.as_ref().unwrap().borrow().val
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    pub fn add_at_head(&self, val: i32) {
        self.add_at_index(0, val);
    }

    /** Append a node of value val to the last element of the linked list. */
    pub fn add_at_tail(&self, val: i32) {
        self.add_at_index(self.len.get(), val);
    }

    // 定位到 index 左边
    #[inline]
    fn locate(&self, index: i32) -> Rc<RefCell<Node>> {
        if index <= self.len.get() / 2 {
            let mut p = self.head.clone().unwrap();
            for _ in 0..index {
                let next = p.borrow().next.clone().unwrap();
                p = next;
            }
            p
        } else {
            let mut p = self.tail.clone().unwrap();
            for _ in 0..=self.len.get() - index {
                let prev = p.borrow().prev.as_ref().unwrap().upgrade().clone().unwrap();
                p = prev;
            }
            p
        }
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    pub fn add_at_index(&self, index: i32, val: i32) {
        if index > self.len.get() || index < 0 {
            return;
        }
        let p = self.locate(index);

        // 记录左右节点
        let next = p.borrow().next.clone().unwrap();
        let prev = p;

        // 新建节点
        let node = Rc::new(RefCell::new(Node::new(val)));
        node.borrow_mut().prev = Some(Rc::downgrade(&prev));
        node.borrow_mut().next = Some(next.clone());

        // 改变左右节点的 next & prev 指针
        next.borrow_mut().prev = Some(Rc::downgrade(&node));
        prev.borrow_mut().next = Some(node);

        self.len.set(self.len.get() + 1);
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    pub fn delete_at_index(&self, index: i32) {
        if index >= self.len.get() || index < 0 {
            return;
        }
        let p = self.locate(index).borrow().next.clone().unwrap();

        let prev = p.borrow().prev.as_ref().unwrap().upgrade().clone().unwrap();
        let next = p.borrow().next.clone().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));

        self.len.set(self.len.get() - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode_test;
    use super::MyLinkedList;
    /// ```python
    /// import re
    ///
    /// a = ["MyLinkedList","get","addAtIndex","get","get","addAtIndex","get","get"]
    /// b = [[],[0],[1,2],[0],[1],[0,1],[0],[1]]
    ///
    /// a = [re.sub(r'([A-Z])', r'_\1', i).lower() for i in a]
    ///
    /// for i, j in zip(a, b):
    ///     print(f'obj.{i}({", ".join(str(n) for n in j)});')
    /// ```
    #[test]
    fn test1() {
        leetcode_test!(
            ["MyLinkedList","addAtHead","addAtTail","addAtIndex","get","deleteAtIndex","get"]
            [[],[1],[3],[1,2],[1],[1],[1]]
            [null,null,null,null,2,null,3]
        );
    }
}
