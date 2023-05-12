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
} 