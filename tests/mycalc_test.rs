// #[path = "../src/mycalc.rs"] mod mycalc; 

extern crate my_rust;
use std::arch::asm;

use my_rust::mycalc::*;

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

/// adding documentation 
/// test_asm1: using the inline assembly feature
#[test]
fn test_asm1() {
        let i: u64 = 3;
        let mut o: u64 = 1;
        assert_eq!(o, 1); 
        assert_ne!(o, i); 
        unsafe {
            asm!(
                "mov {0}, {1}",
                out(reg) o,
                in(reg) i,
            );
        }
        assert_eq!(o, i); 
}

