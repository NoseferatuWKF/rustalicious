struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {

    fn new(val: T) -> Self {
        Node {val, next: None}
    }

    fn length(&self) -> u32 {
        todo!()
    }

    fn insert_at(item: T, index: u32) {
        todo!()
    }
    
    fn remove(item: T) -> T {
        todo!()
    }

    fn remove_at(index: u32) -> T {
        todo!()
    }

    fn append(item: T) {
        todo!()
    }

    fn prepend(item: T) {
        todo!()
    }

    fn get(index: u32) -> T {
        todo!()
    }

}


