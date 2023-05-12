/// returns the sum of the given two i32 numbers
///
/// # Examples
///
/// ```
/// let arg1 = 1;
/// let arg2 = 2;
/// let result = my_rust::my_lib::sum2(arg1, arg2);
///
/// assert_eq!(3, result);
/// ```
pub fn sum2(a: i32, b: i32) -> i32 {
    a + b
}

pub fn mul2(a: i32, b: i32) -> i32 {
    mul21(a, b)
}

pub fn mul21(a: i32, b: i32) -> i32 {
    return a * b;
}

#[cfg(test)]
mod tests {

    use super::mul21;

    #[test]
    fn mul21_test() {
        assert_eq!(mul21(8, 4), 32);
    }
}
