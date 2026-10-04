const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Converts the provided hexadecimal [`char`] into its binary/decimal [`u8`] representation.
///
/// If the char is an invalid hexadecimal character (not 0-9, A-F, a-f), [`Option::None`] is returned instead.
///
/// # Examples
/// ```
/// let number: Option<u8> = cryptopals::hex_char_to_byte('4');
/// let valid: Option<u8> = cryptopals::hex_char_to_byte('d');
/// let invalid: Option<u8> = cryptopals::hex_char_to_byte('x');
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

    return Some(byte);
}

/// Converts the provided [`&str`] representing a hexadecimal number into its binary/decimal [`u8`] representation.
///
/// The number is returned as a [`Vec`] of bytes with the most significant byte coming first.
///
/// If the string contains any invalid hexadecimal characters (see [`cryptopals::hex_char_to_byte`]), an [`Option::None`] is returned instead.
///
/// # Examples
/// ```
/// let result: Option<Vec<u8>> = cryptopals::hex_str_to_bytes("12c3f");
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

/// Converts the provided [`Vec`] of [`u8`] bytes into its base64 [`String`] representation.
///
/// # Examples
/// ```
/// let bytes: Vec<u8> = "Hello World!".as_bytes().to_vec();
/// let result: String = cryptopals::bytes_to_base64(&bytes);
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
