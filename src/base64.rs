pub fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Creating a lookup table to find the index of each character in BASE64_CHARS
    let mut index_map = [255u8; 256];
    for (i, c) in BASE64_CHARS.chars().enumerate() {
        index_map[c as usize] = i as u8;
    }

    if input.is_empty() {
        return Err("Invalid input: empty string");
    }

    let mut input = input.to_string();

    while input.len() % 4 != 0 {
        input.push('=');
    }

    let mut output = Vec::with_capacity((input.len() * 3) / 4);
    let mut buffer: u32 = 0;
    let mut bits_stored = 0;

    for c in input.chars() {
        if c == '=' {
            break;
        }

        let value = index_map[c as usize];

        if value == 255 {
            return Err("Invalid character in input");
        }

        buffer = (buffer << 6) | value as u32;
        bits_stored += 6;

        while bits_stored >= 8 {
            bits_stored -= 8;
            output.push((buffer >> bits_stored) as u8);
            buffer &= (1 << bits_stored) - 1;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64_decode_valid() {
        let input = "SGVsbG8gd29ybGQ="; // "Hello world"
        let expected = b"Hello world".to_vec();
        let result = base64_decode(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_base64_decode_valid2() {
        let input = "UnVzdCBJcyBWZXJ5IE9wdGltYWwgYW5kIEVmZmllY2llbnQ="; // "Rust Is Very Optimal and Effiecient"
        let expected = b"Rust Is Very Optimal and Effiecient".to_vec();
        let result = base64_decode(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_base64_decode_valid3() {
        let input = "RGVjb2RpbmcgU29tZSBCYXNlNjQgZW5jb2RlZCBNZXNzYWdl"; // "Decoding Some Base64 encoded Message"
        let expected = b"Decoding Some Base64 encoded Message".to_vec();
        let result = base64_decode(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_base64_decode_invalid() {
        let input = "InvalidBase64!";
        let result = base64_decode(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_decode_empty() {
        let input = "";
        let result = base64_decode(input);
        assert!(result.is_err());
    }
}
