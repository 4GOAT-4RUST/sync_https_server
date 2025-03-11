pub fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    // Define the Base64 character set (used for encoding and decoding)
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Create a lookup table to quickly find the index of each character in BASE64_CHARS
    // 255 means the character is not a valid Base64 character
    let mut index_map = [255u8; 256]; 
    for (i, c) in BASE64_CHARS.chars().enumerate() {
        index_map[c as usize] = i as u8;
    }

    // Check if the input string is empty
    if input.is_empty() {
        return Err("Invalid input: empty string");
    }

    // Base64 strings must be a multiple of 4 in length
    if input.len() % 4 != 0 {
        return Err("Invalid length: must be a multiple of 4");
    }

    // Convert input to a String so we can modify it
    let mut input = input.to_string();

    // Base64 requires padding with '=' characters, so we add them if needed
    while input.len() % 4 != 0 {
        input.push('='); 
    }

    // Prepare an output buffer with enough space for the decoded data
    let mut output = Vec::with_capacity((input.len() * 3) / 4);

    let mut buffer: u32 = 0; // Temporary storage for decoded bits
    let mut bits_stored = 0; // Keeps track of how many bits are in the buffer

    // Process each character in the input string
    for c in input.chars() {
        // Ignore padding characters ('=') since they don't hold actual data
        if c == '=' {
            break;
        }

        // Get the numerical value of the character from our lookup table
        let value = index_map[c as usize];

        // If the value is 255, it means the character is not a valid Base64 character
        if value == 255 {
            return Err("Invalid character in input");
        }

        // Shift the buffer left by 6 bits (making room for new data) and add the new value
        buffer = (buffer << 6) | value as u32;
        bits_stored += 6; // Keep track of how many bits we have

        // If we have collected at least 8 bits, we can extract a byte
        while bits_stored >= 8 {
            bits_stored -= 8; // Reduce stored bits
            output.push((buffer >> bits_stored) as u8); // Extract the byte and store it
            buffer &= (1 << bits_stored) - 1; // Remove used bits from buffer
        }
    }

    // Return the decoded data as a byte vector
    Ok(output)
}
