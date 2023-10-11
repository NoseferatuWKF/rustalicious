// mod algorithms;
// use crate::algorithms::binary_search::sorted_binary_search;
mod data_structures;
use crate::data_structures::linked_list::LinkedList;

fn main() {
    linked_list();
}

fn linked_list() {
    let mut l: LinkedList<u8> = LinkedList::new();
    l.append(5);
    l.append(4);
    l.append(3);
    dbg!(&l);

    println!("{:?}", l.get(0)); // 5
    println!("{:?}", l.get(1)); // 4

    l.insert_at(6, 0);
    dbg!(&l);

    println!("{:?}", l.get(1)); // 6
    println!("{:?}", l.get(2)); // 4
    println!("{:?}", l.get(3)); // 3

    println!("{:?}", l.remove(4));
    dbg!(&l);

    println!("{:?}", l.get(2)); // 3
    println!("{:?}", l.get(3)); // None
    println!("{:?}", l.remove_at(1));
    dbg!(&l);
}
