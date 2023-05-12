pub fn simple_array1()
{
    println!("sample array1. pass 1");

    let mut array1 = [1, 2, 3, 4, 5];
    for i in array1.iter()
    {
        println!("{}", i);
    }

    println!("sample array1. pass 2");

    let mut pos = 0;
    while pos < array1.len()
    {
        println!("{}", array1[pos]);
        array1[pos] = array1[pos] + 10;
        pos += 1;
    }

    println!("sample array1. pass 3");
    for x in array1.iter()
    {
        println!("{}", x);
    }

    println!("sample array1. strings");

    let str_arr = ["one", "two", "three"];
    for str in str_arr.iter()
    {
        println!("{} : {}", str, str.len());
    }

    println!("sample array1. 2D matrix");

    let matrix1 = [[1,2,3],[4,5,6],[7,8,9]];
    for matrix1_line in matrix1.iter()
    {
        for matrix_item in matrix1_line.iter() {
            println!("{}",matrix_item);
        }
    }

    // pretty print
    for matrix1_line in matrix1.iter()
    {
        println!("{} | {} | {}", matrix1_line[0], matrix1_line[1], matrix1_line[2]);
    }

} 