use std::collections::LinkedList;

pub fn linked_list_sample() {
    let mut linked_list1 = LinkedList::from([2, 0, 1, 8]);
    assert_eq!(linked_list1.len(), 4);
    println!("LinkedList sample1 {:?}", linked_list1);

    linked_list1.push_front(0);
    assert_eq!(linked_list1, LinkedList::from([0, 2, 0, 1, 8]));
    println!("LinkedList after push_front(0) {:?}", linked_list1);

    linked_list1.push_back(4);
    assert_eq!(linked_list1, LinkedList::from([0, 2, 0, 1, 8, 4]));
    println!("LinkedList after push_back(4) {:?}", linked_list1);

    linked_list1.pop_front();
    assert_eq!(linked_list1, LinkedList::from([2, 0, 1, 8, 4]));
    println!("LinkedList after pop_front {:?}", linked_list1);

    linked_list1.pop_back();
    assert_eq!(linked_list1, LinkedList::from([2, 0, 1, 8]));
    println!("LinkedList after pop_back {:?}", linked_list1);
}