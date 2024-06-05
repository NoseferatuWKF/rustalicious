use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Weak<RefCell<Node<K, V>>>>,
    next: Option<Weak<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LRU<K, V> {
    capacity: usize,
    length: usize,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
    lookup: HashMap<K, Rc<RefCell<Node<K, V>>>>,
}

pub trait Operations<K, V> {
    fn new(capacity: usize) -> Self;
    fn get(&self, key: K) -> Option<V>;
    fn put(&mut self, key: K, value: V);
}

impl<K: Eq + Hash + Clone + std::fmt::Debug, V: Eq + Clone + std::fmt::Debug> Operations<K, V> for LRU<K, V> {
    fn new(capacity: usize) -> LRU<K, V> {
        LRU {
            capacity,
            length: 0,
            head: None,
            tail: None,
            lookup: HashMap::new()
        }
    }

    fn get(&self, key: K) -> Option<V> {
        if let Some(node) = self.lookup.get(&key) {
            self.detach(&node);
        }
        None
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.lookup.get_mut(&key) {
            self.head = Some(Rc::clone(&node));

            if let Some(tail) = &self.tail {
                if tail.borrow().value == node.borrow().value {
                    let mut swap = None;
                    if let Some(prev) = &tail.borrow().prev {
                        swap = prev.upgrade();
                    }

                    if let Some(res) = &swap {
                        res.borrow_mut().next = None;
                        res.borrow_mut().prev = Some(Rc::downgrade(&res));
                        self.tail = swap;
                    }
                }
            }

            node.borrow_mut().value = value;
        } else {
            let node = Rc::new(RefCell::new(Node::new(key.clone(), value)));

            if let Some(head) = &self.head {
                node.borrow_mut().next = Some(Rc::downgrade(&head));
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
            } else {
                self.tail = Some(Rc::clone(&node));
            }

            self.head = Some(Rc::clone(&node));

            self.lookup.insert(key, node);
            self.length += 1;

            self.trim_cache();
        }
    }
}

impl<K: Eq + Hash + Clone, V: Eq + Clone> LRU<K,V> {
    // TODO: Figure out how to do mutation for head and tail
    fn detach(&self, node: &Rc<RefCell<Node<K, V>>>) {
        if node.borrow().next.is_some() {
            if let Some(next) = &node.borrow().next {
                let upgrade = next.upgrade();
                if let Some(result) = upgrade {
                    result.borrow_mut().prev = node.borrow_mut().prev.take();
                }
            }
        }

        if node.borrow().prev.is_some() {
            if let Some(prev) = &node.borrow().prev {
                let upgrade = prev.upgrade();
                if let Some(result) = upgrade {
                    result.borrow_mut().next = node.borrow_mut().next.take();
                }
            }
        }

        if let Some(head) = &self.head {
            if head.borrow().value == node.borrow().value {
                let mut swap = None;
                if let Some(next) = &head.borrow().next {
                    swap = next.upgrade();
                }

                if let Some(result) = &swap {
                    result.borrow_mut().prev = None;
                }
            }
        }

        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
    }

    fn trim_cache(&mut self) {
        if self.length <= self.capacity {
            return;
        } else {
            let mut swap = None;
            if let Some(tail) = &self.tail {
                self.lookup.remove(&tail.borrow().key);

                if let Some(prev) = &tail.borrow().prev {
                    swap = prev.upgrade();
                }
            }

            if let Some(res) = &swap {
                self.tail = swap;
            }

            self.length -= 1;
        }
    }
}
