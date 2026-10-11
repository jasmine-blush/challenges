const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Converts the provided hexadecimal [`char`] into its binary/decimal [`u8`] representation.
///
/// If the char is an invalid hexadecimal character (not 0-9, A-F, a-f), [`Option::None`] is returned instead.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::hex_char_to_byte;
///
/// let number: Option<u8> = hex_char_to_byte('4');
/// let valid: Option<u8> = hex_char_to_byte('d');
/// let invalid: Option<u8> = hex_char_to_byte('x');
///
/// assert_eq!(number, Some(4));
/// assert_eq!(valid, Some(13));
/// assert_eq!(invalid, None);
/// ```
pub fn hex_char_to_byte(char: char) -> Option<u8> {
    let mut byte: u8 = char as u8;

    // Only 0-9, A-F and a-f are valid
    if byte > 96 {
        byte -= 87;
        if byte > 15 {
            return None;
        }
    } else if byte > 64 {
        byte -= 55;
        if byte > 15 {
            return None;
        }
    } else if byte > 47 {
        byte -= 48;
        if byte > 9 {
            return None;
        }
    } else {
        return None;
    }

    Some(byte)
}

/// Converts the passed [`&str`] representing a hexadecimal number into its binary/decimal [`u8`] representation.
///
/// The number is returned as a [`Vec`] of bytes with the most significant byte coming first.
///
/// If the string contains any invalid hexadecimal characters (see [`cryptopals::hex_char_to_byte`]), an [`Option::None`] is returned instead.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::hex_str_to_bytes;
///
/// let result: Option<Vec<u8>> = hex_str_to_bytes("12c3f");
/// // decimal: 76863, binary: 00000001 00101100 00111111
///
/// let expected: Vec<u8> = vec![1, 44, 63];
/// assert_eq!(result, Some(expected));
/// ```
pub fn hex_str_to_bytes(input: &str) -> Option<Vec<u8>> {
    let mut result: Vec<u8> = Vec::new();

    let chars: Vec<char> = input.chars().collect();
    let mut start: usize = chars.len();
    while start > 1 {
        let high: u8 = hex_char_to_byte(chars[start - 2])?;
        let low: u8 = hex_char_to_byte(chars[start - 1])?;

        let hex_num: u8 = (high << 4) | low;
        result.push(hex_num);

        start -= 2;
    }
    if start == 1 {
        let hex_num: u8 = hex_char_to_byte(chars[0])?;
        result.push(hex_num);
    }

    result.reverse();

    Some(result)
}

/// Converts the bytes of the passed [`Vec<u8>`] into its base64 [`String`] representation.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::bytes_to_base64;
///
/// let bytes: Vec<u8> = "Hello World!".as_bytes().to_vec();
/// let result: String = bytes_to_base64(&bytes);
///
/// let expected: &str = "SGVsbG8gV29ybGQh";
/// assert_eq!(result, expected);
/// ```
pub fn bytes_to_base64(input: &Vec<u8>) -> String {
    let mut result = String::new();

    let mut group = Vec::with_capacity(3);
    for byte in input {
        group.push(byte);

        if group.len() == 3 {
            let first: u8 = (group[0] & 0xFC) >> 2; // 0x12345678 -> 00123456
            let second: u8 = ((group[0] & 0x3) << 4) | ((group[1] & 0xF0) >> 4); // 0x12345678 1x12345678 -> 00781234
            let third: u8 = ((group[1] & 0xF) << 2) | ((group[2] & 0xC0) >> 6); // 1x12345678 2x12345678 -> 00567812
            let fourth: u8 = group[2] & 0x3F; // 2x12345678 -> 00345678

            result.push(BASE64_CHARS[first as usize]);
            result.push(BASE64_CHARS[second as usize]);
            result.push(BASE64_CHARS[third as usize]);
            result.push(BASE64_CHARS[fourth as usize]);
            group.clear();
        }
    }

    result
}

/// Performs an XOR over all bits of the two [`u8`] slice parameters and returns the resulting vector.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::xor_bytes;
///
/// let first: Vec<u8> = vec![150, 220];    // 10010110 11011100
/// let second: Vec<u8> = vec![123, 89];    // 01111011 01011001
/// let result: Result<Vec<u8>, &'static str> = xor_bytes(&first, &second);
///
/// let expected: Vec<u8> = vec![237, 133]; // 11101101 10000101
/// assert_eq!(result, Ok(expected));
/// ```
pub fn xor_bytes(first: &[u8], second: &[u8]) -> Result<Vec<u8>, &'static str> {
    let len: usize = first.len();
    if len != second.len() {
        return Err("Both vectors must be the same length.");
    }

    let mut result: Vec<u8> = Vec::with_capacity(len);
    let mut i: usize = 0;
    while i < len {
        result.push(first[i] ^ second[i]);
        i += 1;
    }

    Ok(result)
}

/// Performs an XOR over all bytes of the [`u8`] slice with the passed byte and returns the resulting vector.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::xor_bytes_repeating;
///
/// let bytes: Vec<u8> = vec![40, 143, 98];     // 00101000 10001111 01100010
/// let byte: u8 = 55;                          // 001101111
/// let result: Vec<u8> = xor_bytes_repeating(&bytes, byte);
///
/// let expected: Vec<u8> = vec![31, 184, 85];  // 00011111 10111000 01010101
/// assert_eq!(result, expected);
/// ```
pub fn xor_bytes_repeating(bytes: &[u8], byte: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());

    for b in bytes {
        result.push(b ^ byte);
    }

    result
}

/// Converts the passed [`Vec<u8>`] to its hexadecimal representation and returns it as a string.
///
/// The HEX decimals A-Z will be in capital letters.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::bytes_to_hex_string;
///
/// let bytes: Vec<u8> = vec![52, 125, 250]; // 00110100 01111101 11111010
/// let result: String = bytes_to_hex_string(&bytes);
///
/// let expected: String = String::from("347DFA");
/// assert_eq!(result, expected);
/// ```
pub fn bytes_to_hex_string(input: &Vec<u8>) -> String {
    let mut result: String = String::with_capacity(input.len());

    for byte in input {
        let high: u8 = byte >> 4;
        let low: u8 = byte & 15;

        result.push(HEX_CHARS[high as usize]);
        result.push(HEX_CHARS[low as usize]);
    }

    result
}

/// Converts each byte of the passed [`Vec<u8>`] to a [`char`] (unicode) and returns it as a full string.
///
/// # Examples
/// ```
/// use cryptopals::utils::hex::bytes_to_char_string;
///
/// let bytes: Vec<u8> = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33];
/// let result: String = bytes_to_char_string(&bytes);
///
/// let expected: String = String::from("Hello World!");
/// assert_eq!(result, expected);
/// ```
pub fn bytes_to_char_string(input: &Vec<u8>) -> String {
    let mut result: String = String::with_capacity(input.len());

    for byte in input {
        let char = char::from_u32(*byte as u32);

        if let Some(char) = char {
            result.push(char);
        }
    }

    result
}
