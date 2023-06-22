// #[path = "../src/mycalc.rs"] mod mycalc; 

extern crate my_rust;
use my_rust::my_calc::*;

#[test]
fn test_sum1() {
    let a:i32 = 3;
    let b:i32 = 4;
    let s1:i32 = sum1(a, b);

    assert_eq!(s1, 7);
}

#[test]
fn test_mul1() {
    let a:i32 = 3;
    let b:i32 = 4;
    let m1:i32 = mul1(a, b);

    assert_eq!(m1, 12);
}
