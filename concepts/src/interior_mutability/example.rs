use std::rc::Rc; // this is what made multiple ownership possible
use std::cell::RefCell; // this is what made interior mutability possible

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }
}

pub fn refcell_example() {
    let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
    let mut node: Rc<RefCell<Node<i32>>> = Rc::new(RefCell::new(Node::new(69)));

    // this is not deep copy, only increases the reference count
    dll.head = Some(Rc::clone(&node));
    dll.tail = Some(Rc::clone(&node));

    node.borrow_mut().value = 420; // this is made possible by interior mutability

    dbg!("{}", dll); // value for both nodes is now 420
}
