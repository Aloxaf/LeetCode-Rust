//! 第一反应: 既然全是整数, 那就开它个 1000000 的数组
//! 然而 `Box::new([-1; 1000000])` 初始化的时候会 stack overflow,
//! 查了一下说可以使用 `box [-1; 1000000]`, 然而是 unstable feature...
//! 那就用 `vec![-1; 1000000]` 吧, 本地倒没什么问题了, 然而交上去又 MLE ...
//! ...
//! 那就开个 1000 的 Vec, 其余的用链表存吧
//! 然而标准库里的链表没有 remove 方法 ??? (https://github.com/rust-lang/rust/issues/39148
//! 那这玩意儿还有存在的意义吗...

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 根据设计链表那题改造来的双向链表
#[derive(Clone, Debug)]
pub struct MyLinkedList {
    len: i32,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

#[derive(Clone, Debug)]
struct Node {
    pub key: i32,
    pub val: i32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

impl MyLinkedList {
    pub fn new() -> Self {
        let ret = MyLinkedList {
            len: 0,
            head: Some(Rc::new(RefCell::new(Node::new(0, 0)))),
            tail: Some(Rc::new(RefCell::new(Node::new(0, 0)))),
        };
        ret.head.as_ref().unwrap().borrow_mut().next = ret.tail.clone();
        ret.tail.as_ref().unwrap().borrow_mut().prev = Some(Rc::downgrade(ret.head.as_ref().unwrap()));
        ret
    }

    pub fn push(&mut self, key: i32, val: i32) {
        if let Some(p) = self.find_by_key(key) {
            p.borrow_mut().val = val;
            return;
        }

        let p = self.tail.clone().unwrap();
        let p = p.borrow().prev.as_ref().unwrap().upgrade().clone().unwrap();

        // 记录左右节点
        let next = p.borrow().next.clone().unwrap();
        let prev = p;

        // 新建节点
        let node = Rc::new(RefCell::new(Node::new(key, val)));
        node.borrow_mut().prev = Some(Rc::downgrade(&prev));
        node.borrow_mut().next = Some(next.clone());

        // 改变左右节点的 next & prev 指针
        next.borrow_mut().prev = Some(Rc::downgrade(&node));
        prev.borrow_mut().next = Some(node);

        self.len += 1;
    }

    #[inline]
    fn find_by_key(&self, key: i32) -> Option<Rc<RefCell<Node>>> {
        let mut p = self.head.clone().unwrap();
        for _ in 0..self.len {
            let next = p.borrow().next.clone().unwrap();
            p = next;
            if p.borrow().key == key {
                return Some(p);
            }
        }
        None
    }

    pub fn get(&self, key: i32) -> i32 {
        self.find_by_key(key).map(|p| p.borrow().val).unwrap_or(-1)
    }

    pub fn remove_by_key(&mut self, key: i32) {
        if let Some(p) = self.find_by_key(key) {
            let prev = p.borrow().prev.as_ref().unwrap().upgrade().clone().unwrap();
            let next = p.borrow().next.clone().unwrap();

            prev.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(Rc::downgrade(&prev));
            self.len -= 1;
        }
    }
}

pub struct MyHashMap {
    map: Vec<Option<MyLinkedList>>,
}

/** You can modify the type of `self` for your need. */
impl MyHashMap {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            map: vec![None; 1000],
        }
    }

    /** value will always be non-negative. */
    pub fn put(&mut self, key: i32, val: i32) {
        let idx = (key % 1000) as usize;
        if self.map[idx].is_none() {
            self.map[idx] = Some(MyLinkedList::new());
        }
        self.map[idx].as_mut().unwrap().push(key, val);
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        let idx = (key % 1000) as usize;
        match self.map[idx].as_ref() {
            None => -1,
            Some(sub_map) => sub_map.get(key)
        }
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    pub fn remove(&mut self, key: i32) {
        let idx = (key % 1000) as usize;
        if let Some(sub_map) = self.map[idx].as_mut() {
            sub_map.remove_by_key(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MyHashMap, MyLinkedList};
    use crate::leetcode_test;

    #[test]
    fn test1() {
        leetcode_test!(
            ["MyHashMap","put","put","get","get","put","get", "remove", "get"]
            [[],[1,1],[2,2],[1],[3],[2,1],[2],[2],[2]]
            [null,null,null,1,-1,null,1,null,-1]
        );
    }

    #[test]
    fn test_linkedlist() {
        let mut obj = MyLinkedList::new();
        obj.push(1, 11);
        obj.push(2, 12);
        obj.push(3, 13);
        obj.push(1, 10);
        assert_eq!(obj.get(1), 10);
        obj.remove_by_key(1);
        assert_eq!(obj.get(1), -1);
        obj.remove_by_key(2);
        obj.remove_by_key(3);
        assert_eq!(obj.get(1), -1);
    }
}
