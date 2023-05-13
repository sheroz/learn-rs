pub fn fibo(number: u32) -> u32 {
    if number <= 1 {
        return number;
    }

    fibo(number - 1) + fibo(number - 2)
}

#[cfg(test)]
mod tests {
    use super::fibo;
    #[test]
    fn fibo_test() {
        let s = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let r: Vec<_> = (0..s.len()).map(|n| fibo(n as u32)).collect();
        assert_eq!(&s[..], &r[..]);
    }
}
