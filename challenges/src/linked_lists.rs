use std::collections::LinkedList;



pub fn create_linked_list() {
    let mut ll = LinkedList::new();
    ll.push_back('a');    
    ll.push_back('b');
    ll.push_back('c');
    println!("{:?}", ll);
}