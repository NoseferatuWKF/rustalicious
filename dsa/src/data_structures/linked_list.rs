pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
        }
    }

    pub fn insert_at(&mut self, item: T, mut index: usize) {
        match self.length > index {
            true => {
                let mut curr = self.head.as_mut();
                while index > 0 {
                    curr = curr.unwrap().next.as_mut();
                    index -= 1;
                }
                if let Some(ref mut node) = curr {
                    let mut new_node = Node {val: item, next: None};
                    if node.next.is_some() {
                        new_node.next = node.next.take();
                    }
                    node.next = Some(Box::new(new_node));
                }
                self.length += 1;
                return;
            },
            false => (),
        }
    }
    
    pub fn remove(&mut self, item: T) -> Option<T> {
        if let Some(ref mut head) = self.head {
            if head.val == item {
                if let Some(mut n) = self.head.take() {
                    self.head = n.next.take();
                    self.length -= 1;
                    return Some(n.val);
                }
            }
        }
        match self.head {
            Some(ref mut head) => {
                let mut curr = head;
                while curr.next.is_some() {
                    if let Some(ref next) = curr.next {
                        if next.val == item {
                            if let Some(mut n) = curr.next.take() {
                                curr.next = n.next.take();
                                self.length -= 1;
                                return Some(n.val);
                            }
                        }
                    }
                    curr = curr.next.as_mut().unwrap();
                }
                return None;
            },
            None => (),
        }
        None
    }

    pub fn remove_at(&mut self, mut index: usize) -> Option<T> {
        match self.length > index {
            true => {
                if index == 0 {
                    let curr = self.head.take();
                    if let Some(mut node) = curr {
                        if node.next.is_some() {
                            self.head = node.next.take();
                            return Some(node.val);
                        }
                    }
                    return None;
                } else {
                    if let Some(ref mut head) = self.head {
                        let mut curr = head;
                        while index > 1 {
                            if let Some(ref mut node) = curr.next {
                                curr = node;
                            }
                            index -= 1;
                        }
                        if let Some(mut n) = curr.next.take() {
                            curr.next = n.next.take();
                            self.length -= 1;
                            return Some(n.val);
                        }
                    }
                    self.length += 1;
                    return None;
                }
            },
            false => None,
        }
    }

    pub fn append(&mut self, item: T) {
        let node = Node {val: item, next: None};
        match self.head {
            Some(ref mut head) => {
                let mut curr = head;
                while curr.next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                curr.next = Some(Box::new(node));
            },
            None => {
                self.head = Some(Box::new(node));
            },
        }
        self.length += 1;
    }

    pub fn get(&mut self, mut index: usize) -> Option<&T> {
        match self.length > index {
            true => {
                let mut curr = self.head.as_ref();
                while index > 0 {
                    curr = curr.unwrap().next.as_ref();
                    index -= 1;
                }
                if let Some(ref node) = curr {
                    return Some(&node.val);
                }
                return None;
            },
            false => None,
        }
    }
}
