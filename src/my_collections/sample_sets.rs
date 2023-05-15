use std::collections::{HashSet, BTreeSet};

pub fn hashset_sample() {
    let mut hashset1 = HashSet::from(["1-one","2-two","3-three"]);
    assert_eq!(hashset1.len(), 3);
    assert!(hashset1.contains("1-one"));
    assert!(hashset1.contains("2-two"));
    assert!(!hashset1.contains("6-six"));
    println!("HashSet sample: {:?}", hashset1);

    hashset1.insert("0-zero");
    hashset1.insert("4-four");
    println!("HashSet sample: {:?}", hashset1);
}

pub fn btreeset_sample() {
    let mut btreeset1 = BTreeSet::from(["1-one","2-two","3-three"]);
    assert_eq!(btreeset1.len(), 3);
    assert!(btreeset1.contains("1-one"));
    assert!(btreeset1.contains("2-two"));
    assert!(!btreeset1.contains("6-six"));
    println!("BTreeSet sample: {:?}", btreeset1);

    btreeset1.insert("0-zero");
    btreeset1.insert("4-four");
    println!("BTreeSet sample: {:?}", btreeset1);
}