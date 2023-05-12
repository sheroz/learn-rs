// #[path = "../src/mycalc.rs"] mod mycalc; 

extern crate my_rust;
use my_rust::my_lib::sum2;
use my_rust::my_lib::mul2;

#[test]
fn test_sum2() {
    let a:i32 = 3;
    let b:i32 = 4;
    let s1:i32 = sum2(a, b);

    assert_eq!(s1, 7);
}

#[test]
fn test_mul2() {
    let a:i32 = 3;
    let b:i32 = 4;
    let m1:i32 = mul2(a, b);

    assert_eq!(m1, 12);
}
