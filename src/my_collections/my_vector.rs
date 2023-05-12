pub fn vector1(){
    println!("\nVectors:");

    let mut vec1 = vec![1, 2, 3];
    print_my_vector(&vec1);

    vec1.push(41);
    vec1.push(42);

    print_my_vector(&vec1);

    for item in vec1.iter_mut() {
        *item += 10;
    }

    print_my_vector(&vec1);
}

fn print_my_vector(myvector: &Vec<i32>) {
    println!("\nsize: {}", myvector.len());
    for item in myvector.iter() {
        print!("{} " , item);
    }
    println!("");
}
