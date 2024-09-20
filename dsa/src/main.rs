mod algorithms;
mod data_structures;

fn main() {
    use algorithms::binary_search::binary_search;
    use data_structures::linked_list::LinkedList;
    use data_structures::lru::{LRU, Operations};
    use data_structures::doubly_linked_list::DoublyLinkedList;
    // linked_list();
    // lru();
}

// fn doubly_linked_list() {
//     let mut dll: DoublyLinkedList<u32> = DoublyLinkedList::new();
// }

// fn lru() {
//     let mut lru: LRU<i32, String> = LRU::new(3);
//
//     lru.put(0, String::from("abc"));
//     lru.put(1, String::from("def"));
//
//     let mut res = lru.get(0);
//     dbg!(&res);
//     lru.put(2, String::from("kjl"));
//     lru.put(1, String::from("res"));
//     lru.put(3, String::from("wow"));
//
//     res = lru.get(0);
//     dbg!(&res);
//
//     dbg!(&lru);
// }

// fn linked_list() {
//     let mut l: LinkedList<u8> = LinkedList::new();
//     l.append(5);
//     l.append(4);
//     l.append(3);
//
//     println!("{:?}", l.get(0)); // 5
//     println!("{:?}", l.get(1)); // 4
//
//     l.insert_at(6, 0);
//
//     println!("{:?}", l.get(1)); // 6
//     println!("{:?}", l.get(2)); // 4
//     println!("{:?}", l.get(3)); // 3
//
//     println!("{:?}", l.remove(4));
//
//     println!("{:?}", l.get(2)); // 3
//     println!("{:?}", l.get(3)); // None
//     println!("{:?}", l.remove_at(1));
// }
