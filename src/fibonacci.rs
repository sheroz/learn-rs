pub fn fibonacci(number: u32) -> u32 {
    if number <= 1 {
        return number;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}

#[cfg(test)]
mod tests {
    use super::fibonacci;
    #[test]
    fn fibonacci_test() {
        let s = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let r: Vec<_> = (0..s.len()).map(|n| fibonacci(n as u32)).collect();
        assert_eq!(&s[..], &r[..]);
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
