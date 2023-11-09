pub fn array_sample() {
    println!("sample array1. iterating using iter()");
    let mut array1 = [11, 22, 33, 44, 55];
    for i in array1.iter() {
        println!("{}", i);
    }

    println!("{:?}", array1);

    let mut pos = 0;
    while pos < array1.len() {
        array1[pos] = array1[pos] + 10;
        pos += 1;
    }
    println!("sample array1. accessing by index in while {:?}", array1);


    let str_arr = ["one", "two", "three"];
    println!("array of strings {:?}", str_arr);
    for str in str_arr.iter() {
        println!("{} : {}", str, str.len());
    }

    let matrix1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Sample array1. 2D matrix {:?}", matrix1);

    println!("sample array1. pretty print of 2D matrix");
    for matrix1_line in matrix1.iter() {
        println!(
            "{} | {} | {}",
            matrix1_line[0], matrix1_line[1], matrix1_line[2]
        );
    }
}

pub fn array_slice_sample() {
    println!("\n***\n\nArray slice samples");

    let array1 = [11, 22, 33, 44, 55];
    println!("Sample array1. 2D matrix {:?}", array1);


    let slice1 = &array1[2..4]; // elements from index 2 untill 4 (values 33 and 44)
    println!("{} | {}", slice1[0], slice1[1]);
    assert_eq!(slice1, [33, 44]);
    println!("Taking slice... {:?}", slice1);

    // Mapping sliced items...
    let slice2: Vec<i32> = slice1.iter().map(|x| x+1).collect();
    assert_eq!(slice2, [34, 45]);
    println!("Map result {:?}", slice2);

    let sum1: i32 = slice2.iter().sum();
    assert_eq!(sum1, 79);
    println!("\nSlice sum: {}", sum1);
}