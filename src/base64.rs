// links:
// https://www.linkedin.com/posts/rustjobs-dev_base64-implementation-in-rust-activity-7064862496720924672-fN7y
// https://keyboardsmash.dev/posts/implementing-base64-algorithm-in-rust/
pub fn encode(input: &[u8]) -> String {
    const BASE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = Vec::new();
    let input_len = input.len();

    for i in (0..input_len).step_by(3) {
        let a = input.get(i).unwrap();
        let b = input.get(i + 1).unwrap_or(&0);
        let c = input.get(i + 2).unwrap_or(&0);

        let enc1 = (a >> 2) as usize;
        let enc2 = (((a & 0x3) << 4) | (b >> 4)) as usize;
        let enc3 = (((b & 0xf) << 2) | (c >> 6)) as usize;
        let enc4 = (c & 0x3f) as usize;

        output.push(BASE_CHARS[enc1]);
        output.push(BASE_CHARS[enc2]);
       
        output.push(BASE_CHARS[enc3]);
        output.push(BASE_CHARS[enc4]);
    }

    let output_len = output.len();
    let padding_len = match input_len % 3 {
        1 => 2, // Add two padding
        2 => 1, // Add one paddings
        _ => 0, // No paddings needed
    };

    for i in 0..padding_len {
        output[output_len - 1 - i] = b'=';
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let encoded = encode(b"Rust");
        assert_eq!("UnVzdA==", encoded);
    }

    #[test]
    fn test_base64_encode_has_padding() {
        let encoded = encode(b"Rust");
        assert!(encoded.ends_with("=="));
    }
}