
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

}

impl LRUCache {

    pub fn new(capacity: i32) -> Self {

    }

    pub fn get(&mut self, key: i32) -> i32 {

    }

    pub fn put(&mut self, key: i32, value: i32) {

    }
}