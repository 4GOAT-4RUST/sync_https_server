pub fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // creating a lookup table for the Base64 characters
    let mut index_map = [0; 256];
    for (i, c) in base64_chars.chars().enumerate() {
        // Allocating Corresponding Numerical Values to their characters
        index_map[c as usize] = i as u8;
    }

    // Removing Padding Characters
    let input = input.trim_end_matches("=");

    // Ensuring the length is a multiple of 4
    if input.len() % 4 != 0 {
        return Err("Input must be a multiple of 4");
    }

    let mut output: Vec<u8> = Vec::new();
    let mut buffer = 0;
    let mut buffer_length: usize = 0;

    // Decoding the Base64 characters into their Binary Format
    for c in input.chars() {
        let index = index_map[c as usize];
        if index == 0 && c != 'A' {
            return Err("Invalid Base64 character");
        }

        buffer = (buffer << 6) | (index as u32);
        buffer_length += 6;

        if buffer_length >= 8 {
            output.push((buffer >> (buffer_length - 8)) as u8);
            buffer_length -= 8;
        }
    }

    Ok(output)
}
