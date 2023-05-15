use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    email: String,
    name: String,
    address: String,
    age: u8,
}

fn create_users() -> Vec<User> {
    
    let mut users: Vec<User> = Vec::with_capacity(3);

    users.push(User {
        email: "user1@mail.com".to_string(),
        name: "user1".to_string(),
        address: "address1".to_string(),
        age: 21,
    });

    users.push(User {
        email: "user3@mail.com".to_string(),
        name: "user3".to_string(),
        address: "address3".to_string(),
        age: 21,
    });

    users.push(User {
        email: "user2@mail.com".to_string(),
        name: "user2".to_string(),
        address: "address2".to_string(),
        age: 40,
    });

    users
}

pub fn hash_map_sample() {
    let mut hash_map1: HashMap<String, User> = HashMap::new();

    let users = create_users();
    for user in users {
        hash_map1.insert(user.email.clone(), user);
    }

    assert_eq!(hash_map1.len(), 3);
    println!("HashMap sample1 {:?}", hash_map1);

    // get a value from hashmap
    let k = "user2@mail.com".to_string();
    let v = hash_map1.get(&k);
    assert!(v.is_some());
    println!("HashMap value: {:?}", v);
    let u = v.unwrap();
    println!("User's email: {}", u.email);
    println!("User's name: {}", u.name);
    println!("User's address: {}", u.address);
    println!("User's age: {}", u.age);
}

pub fn tree_map_sample() {
    let users = create_users();
    let mut btree_map1: BTreeMap<String, User> = BTreeMap::new();

    for user in users {
        btree_map1.insert(user.email.clone(), user);
    }

    assert_eq!(btree_map1.len(), 3);
    println!("BTreeMap sample1 {:?}", btree_map1);

    // get a value from hashmap
    let k = "user2@mail.com".to_string();
    let v = btree_map1.get(&k);
    assert!(v.is_some());
    println!("BTreeMap value: {:?}", v);
    let u = v.unwrap();
    println!("User's email: {}", u.email);
    println!("User's name: {}", u.name);
    println!("User's address: {}", u.address);
    println!("User's age: {}", u.age);
}
