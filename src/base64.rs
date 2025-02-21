pub fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    const BASE64_TABLE: [u8; 128] = {
        let mut table = [255; 128];
        let base64_chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i = 0;
        while i < base64_chars.len() {
            table[base64_chars[i] as usize] = i as u8;
            i += 1;
        }
        table
    };

    let input = input.trim_end_matches('=');
    if input
        .chars()
        .any(|c| c as u8 >= 128 || BASE64_TABLE[c as usize] == 255)
    {
        return Err("Invalid Base64 string");
    }

    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for c in input.bytes() {
        let val = BASE64_TABLE[c as usize] as u32;
        buffer = (buffer << 6) | val;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            output.push((buffer >> bits_collected) as u8);
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::base64_decode;

    #[test]

    fn test_b64_0() {
        let encoded = "SGVsbG8gd29ybGQ=";

        match base64_decode(encoded) {
            Ok(decoded) => println!("{}", String::from_utf8_lossy(&decoded)),
            Err(e) => println!("Error: {}", e),
        }
    }
}
