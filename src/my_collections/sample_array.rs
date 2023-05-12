pub fn simple_array1() {
    println!("sample array1. pass 1");

    let mut array1 = [11, 22, 33, 44, 55];
    for i in array1.iter() {
        println!("{}", i);
    }

    println!("sample array1. pass 2");

    let mut pos = 0;
    while pos < array1.len() {
        println!("{}", array1[pos]);
        array1[pos] = array1[pos] + 10;
        pos += 1;
    }

    println!("sample array1. pass 3");
    for x in array1.iter() {
        println!("{}", x);
    }

    println!("sample array1. strings");

    let str_arr = ["one", "two", "three"];
    for str in str_arr.iter() {
        println!("{} : {}", str, str.len());
    }

    println!("sample array1. 2D matrix");

    let matrix1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for matrix1_line in matrix1.iter() {
        for matrix_item in matrix1_line.iter() {
            println!("{}", matrix_item);
        }
    }

    println!("sample array1. pretty print of 2D matrix");
    for matrix1_line in matrix1.iter() {
        println!(
            "{} | {} | {}",
            matrix1_line[0], matrix1_line[1], matrix1_line[2]
        );
    }
}

pub fn array_slice() {
    println!("\n***\n\nArray slice samples");

    let array1 = [11, 22, 33, 44, 55];
    for i in array1.iter() {
        println!("{}", i);
    }

    println!("\nTaking slice...");
    let slice1 = &array1[2..4]; // elements from index 2 untill 4 (values 33 and 44)
    println!("{} | {}", slice1[0], slice1[1]);
    assert_eq!(slice1, vec![33, 44]);

    println!("\nMapping sliced items...");
    let slice2: Vec<i32> = slice1.iter().map(|x| x+1).collect();
    assert_eq!(slice2, vec![34, 45]);

    println!("\nMap result");
    for x in slice2.iter() {
        println!("{}", x);
    }

    let sum1: i32 = slice2.iter().sum();
    assert_eq!(sum1, 79);
    println!("\nSlice sum: {}", sum1);

}