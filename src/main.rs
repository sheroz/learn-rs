
use my_rust::sum2;
use my_rust::mul2;

mod mycalc;

use mycalc::sum1;
use mycalc::mul1;

fn main() {
    let a:i32 = 3;
    let b:i32 = 4;

    let s1:i32 = sum1(a, b);
    let m1:i32 = mul1(a, b);

    println!("sum1({}, {}) = {}", a, b, s1);
    println!("mul1({}, {}) = {}", a, b, m1);

    let s2:i32 = sum2(a, b);
    let m2:i32 = mul2(a, b);

    println!("sum2({}, {}) = {}", a, b, s2);
    println!("mul2({}, {}) = {}", a, b, m2);
}


