use std::collections::HashMap;

#[derive(Debug)]
struct User {
    email: String,
    name: String,
    address: String,
    age: u8,
}
fn create_users() -> [User;3] {
    let user1 = User {
        email: "user1@mail.com".to_string(),
        name: "user1".to_string(),
        address: "address1".to_string(),
        age: 21,
    };


    let user2 = User {
        email: "user2@mail.com".to_string(),
        name: "user2".to_string(),
        address: "address2".to_string(),
        age: 21,
    };

    let user3 = User {
        email: "user3@mail.com".to_string(),
        name: "user3".to_string(),
        address: "address3".to_string(),
        age: 40,
    };

    [user1, user2, user3]
}

pub fn hash_map_sample() {
    let users = create_users();
    let mut hash_map1: HashMap<&String, &User> = HashMap::new();
    /*
    for &user in users {
        hash_map1.insert(&user.email, &user);
    }
    */

    assert_eq!(hash_map1.len(), 3);
    println!("HashMap sample1 {:?}", hash_map1);

    // get a value from hashmap
    let k = "user2@mail.com".to_string();
    let v = hash_map1.get(&k);
    assert!(v.is_some());
    println!("HashMap got value {:?}", v);
    let u = v.unwrap();
    println!("User's email: {}", u.email);
    println!("User's name: {}", u.name);
    println!("User's address: {}", u.address);
    println!("User's age: {}", u.age);

}
