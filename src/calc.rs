use crate::my_lib::mul2;
use crate::my_lib::sum2;

pub fn sum1(a: i32, b: i32) -> i32 {
    a + b
}

pub fn mul1(a: i32, b: i32) -> i32 {
    a * b
}

pub fn calc_sample() {
    let a: i32 = 3;
    let b: i32 = 4;

    let s1: i32 = sum1(a, b);
    let m1: i32 = mul1(a, b);

    println!("sum1({}, {}) = {}", a, b, s1);
    println!("mul1({}, {}) = {}", a, b, m1);

    let s2: i32 = sum2(a, b);
    let m2: i32 = mul2(a, b);

    println!("sum2({}, {}) = {}", a, b, s2);
    println!("mul2({}, {}) = {}", a, b, m2);
}
