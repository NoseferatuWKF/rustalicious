use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
    prev: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    length: usize,
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, mut node: Node<T>) {
        // let mut node = Node::new(value);
        if let Some(head) = &self.head {
            node.next = Some(head.clone());
            let rc_node = Rc::new(node);
            // head.prev = Some(rc_node.clone());
            // head = &mut rc_node.clone();
        } else {
            let rc_node = Rc::new(node);
            self.head = Some(rc_node.clone());
        }
    }
}
