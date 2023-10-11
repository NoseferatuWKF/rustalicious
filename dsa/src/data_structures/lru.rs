use std::boxed::Box;
use std::option::Option;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node<T> where T: Clone {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LRU<K: std::cmp::Eq + std::hash::Hash, V: std::clone::Clone> {
    capacity: usize,
    length: usize,
    head: Option<Box<Node<V>>>,
    tail: Option<Box<Node<V>>>,
    lookup: HashMap<K, Box<Node<V>>>,
}

trait Operations<K: std::cmp::Eq + std::hash::Hash, V: std::clone::Clone> {
    fn new(capacity: usize) -> Self;
    fn get(&mut self, key: &K) -> Option<&V>;
    fn update(&mut self, key: &K, value: V);
}

impl<K: std::cmp::Eq + std::hash::Hash, V: std::clone::Clone> Operations<K, V> for LRU<K, V> {
    fn new(capacity: usize) -> Self {
        LRU {
            capacity,
            length: 0,
            head: None,
            tail: None,
            lookup: HashMap::<K, Box<Node<V>>>::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        match self.lookup.get_mut(key) {
            Some(node) => {
                if let Some(ref mut next) = node.next {
                    next.prev = node.prev.take();
                }
                if let Some(ref mut prev) = node.prev {
                    prev.next = node.next.take();
                }
                if let Some(ref mut head) = self.head {
                    head.prev = Some(node.clone());
                    self.head = Some(node.clone());
                    node.next = self.head.take();
                }
                return Some(&node.value)
            },
            None => None 
        }
    }

    fn update(&mut self, key: &K, value: V) {
        todo!();
    }
}
