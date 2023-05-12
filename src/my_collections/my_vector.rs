pub fn vector1(){
    println!("\n***\n\nVectors:");

    let mut numbers = Vec::from([-1, 0, 1, 100]);
    assert_eq!(numbers.len(), 4);
    assert_eq!(numbers, vec![-1, 0, 1, 100]);
    let x = numbers.get(3);
    assert_eq!(*x.unwrap(), 100);

    numbers[3] = 200;    
    assert_eq!(numbers, vec![-1, 0, 1, 200]);

    let mut days: Vec<&str> = Vec::new();
    days.push("Monday");
    days.push("Tuesday");
    days.push("Wednesday");
    assert_eq!(days.len(), 3);

    let mut vec1 = vec![1, 2, 3];
    print_my_vector("Sample vector1", &vec1);

    vec1.push(4);
    vec1.push(5);
    assert_eq!(vec1, vec![1,2,3,4,5]);
    print_my_vector("After Push(4), Push(5)", &vec1);

    vec1.pop();
    assert_eq!(vec1, vec![1,2,3,4]);
    print_my_vector("After Pop()", &vec1);

    for item in vec1.iter_mut() {
        *item += 10;
    }
    assert_eq!(vec1, vec![11,12,13,14]);
    print_my_vector("After +10", &vec1);

    vec1.remove(2);
    assert_eq!(vec1, vec![11,12,14]);
    print_my_vector("After remove(2)",&vec1);

    vec1.insert(1,77);
    assert_eq!(vec1, vec![11,77,12,14]);
    print_my_vector("After insert(1,77)",&vec1);

}

fn print_my_vector(msg: &str, myvector: &Vec<i32>) {
    println!("{}", msg);
    println!("size: {}", myvector.len());
    for item in myvector.iter() {
        print!("{} " , item);
    }
    println!("\n");
}
