use std::boxed::Box;
use std::option::Option;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LRU<K: std::cmp::Eq + std::hash::Hash, V> {
    capacity: usize,
    length: usize,
    head: Option<Box<Node<V>>>,
    tail: Option<Box<Node<V>>>,
    lookup: HashMap<K, Box<Node<V>>>,
}

trait Operations<K: std::cmp::Eq + std::hash::Hash, V> {
    fn new(capacity: usize) -> Self;
    fn get(&mut self, key: &K) -> Option<&V>;
    fn update(&mut self, key: &K, value: V);
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Operations<K, V> for LRU<K, V> {
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
        todo!();
    }

    fn update(&mut self, key: &K, value: V) {
        todo!();
    }
}
