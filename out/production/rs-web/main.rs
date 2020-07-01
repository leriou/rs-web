mod corner;

use corner::inner;
use std::collections::HashMap;
use utils::tools;
use std::rc::Rc;
use std::cell::RefCell;

type ORRN = Option<Rc<RefCell<Node>>>;

struct Node {
    key: i32,
    value: i32,
    next: ORRN,
    prev: ORRN,
}

struct LRUCache {
    capacity: i32,
    head: ORRN,
    tail: ORRN,
    map: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::with_capacity(capacity as usize),
            tail: None,
            head: None,
            capacity,
        }
    }

    pub fn move_to_head(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (&prev, &next) {
            (None, None) => {
                // this is the only node in list, so do nothng
            }
            (Some(_), None) => {
                // this node is already the head, so do nothing
            }
            (None, Some(next)) => {
                // this is the tail, move it to front
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                next.borrow_mut().prev = None;
                self.tail = Some(Rc::clone(next));

                let prev_head = self.head.as_ref().unwrap();
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_head));
                self.head = Some(Rc::clone(node));
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;

                prev.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::clone(prev));

                let prev_head = self.head.as_ref().unwrap();
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_head));
                self.head = Some(Rc::clone(node));
            }
        }
    }

    fn push_front(&mut self, node: &Rc<RefCell<Node>>) {
        match &self.head {
            None => {
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
            Some(prev_head) => {
                Rc::clone(node).borrow_mut().prev = Some(Rc::clone(prev_head));
                prev_head.borrow_mut().next = Some(Rc::clone(node));
                self.head = Some(Rc::clone(node));
            }
        }
    }

    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(tail) = self.tail.as_ref().map(|a| a.clone()) {
            if let Some(tail_next) = tail.borrow().next.as_ref() {
                tail_next.borrow_mut().prev = None;
                self.tail = Some(Rc::clone(tail_next));
            }
            tail.borrow_mut().next = None;
            tail.borrow_mut().prev = None;
            return Some(tail);
        }
        None
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(k) = self.map.get(&key) {
            self.move_to_head(k);
            k.borrow().value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(k) = self.map.get(&key) {
            self.move_to_head(k);
            k.borrow_mut().value = value;
        } else {
            // remove tail node if needed
            if self.size >= self.capacity {
                if let Some(prev_tail) = self.list.remove_tail() {
                    self.map.remove(&prev_tail.borrow().key);
                };
            }
            // add node to list head
            let node = Rc::new(RefCell::new(Node {
                prev: None,
                next: None,
                key,
                value,
            }));
            self.push_front(&node);
            // update hashmap
            self.map.insert(key, node);
            // update size
            self.size += 1;
        }
    }
}

fn main() {
//    inner::test::test();
//    println!("tools -> {}", tools::test());
}
