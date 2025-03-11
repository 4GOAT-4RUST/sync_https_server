pub fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut index_map = [255u8; 256]; // 255 represents an invalid character
    for (i, c) in BASE64_CHARS.chars().enumerate() {
        index_map[c as usize] = i as u8;
    }

    if input.is_empty() {
        return Err("Invalid input: empty string");
    }

    if input.len() % 4 != 0 {
        return Err("Invalid length: must be a multiple of 4");
    }

    let mut input = input.to_string();
    while input.len() % 4 != 0 {
        input.push('='); // Ensure proper padding
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
