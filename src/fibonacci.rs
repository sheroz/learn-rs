
#[inline]
pub fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

#[inline]
pub fn fibonacci_iterative(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;    
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

#[cfg(test)]
mod tests {
    #[test]
    fn fibonacci_recursive_test() {
        let s = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let r: Vec<_> = (0..s.len()).map(|n| super::fibonacci_recursive(n as u32)).collect();
        assert_eq!(r, s);
    }

    #[test]
    fn fibonacci_iterative_test() {
        let s = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let r: Vec<_> = (0..s.len()).map(|n| super::fibonacci_iterative(n as u32)).collect();
        assert_eq!(r, s);
    }
    /*
    #[bench]
    fn bench_fibonacci_classic(b: &mut Bencher) {
        b.iter(|| {
            for n in 0..100 {
                fibonacci(n);
            }
        });
    }
    */
}
