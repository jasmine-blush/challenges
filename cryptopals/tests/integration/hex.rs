use cryptopals::utils::hex;
use std::assert_matches;

#[test]
fn test_hex_char_to_byte() {
    let chars: Vec<char> = vec![
        '4', '9', '2', '7', '6', 'd', '2', '0', '6', 'b', '6', '9', '6', 'c', '6', 'c', '6', '9',
        '6', 'e', '6', '7', '2', '0', '7', '9', '6', 'f', '7', '5', '7', '2', '2', '0', '6', '2',
        '7', '2', '6', '1', '6', '9', '6', 'e', '2', '0', '6', 'c', '6', '9', '6', 'b', '6', '5',
        '2', '0', '6', '1', '2', '0', '7', '0', '6', 'f', '6', '9', '7', '3', '6', 'f', '6', 'e',
        '6', 'f', '7', '5', '7', '3', '2', '0', '6', 'd', '7', '5', '7', '3', '6', '8', '7', '2',
        '6', 'f', '6', 'f', '6', 'd',
    ];
    let result: Vec<Option<u8>> = chars
        .iter()
        .map(|char| hex::hex_char_to_byte(*char))
        .collect();

    let _expected: Vec<u8> = vec![
        4, 9, 2, 7, 6, 13, 2, 0, 6, 11, 6, 9, 6, 12, 6, 12, 6, 9, 6, 14, 6, 7, 2, 0, 7, 9, 6, 15,
        7, 5, 7, 2, 2, 0, 6, 2, 7, 2, 6, 1, 6, 9, 6, 14, 2, 0, 6, 12, 6, 9, 6, 11, 6, 5, 2, 0, 6,
        1, 2, 0, 7, 0, 6, 15, 6, 9, 7, 3, 6, 15, 6, 14, 6, 15, 7, 5, 7, 3, 2, 0, 6, 13, 7, 5, 7, 3,
        6, 8, 7, 2, 6, 15, 6, 15, 6, 13,
    ];
    assert_matches!(result, _expected);
}

#[test]
fn test_parse_hex_str() {
    let result: Option<Vec<u8>> = hex::hex_str_to_bytes(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
    );

    let expected: Vec<u8> = vec![
        73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98, 114,
        97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110, 111, 117,
        115, 32, 109, 117, 115, 104, 114, 111, 111, 109,
    ];
    assert_eq!(result, Some(expected));
}

#[test]
fn test_bytes_to_base64() {
    let bytes: Vec<u8> = vec![
        73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98, 114,
        97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110, 111, 117,
        115, 32, 109, 117, 115, 104, 114, 111, 111, 109,
    ];
    let result: String = hex::bytes_to_base64(&bytes);

    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(result, expected);
}

#[test]
fn test_xor_bytes() {
    let first: Vec<u8> = vec![
        28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28,
    ];
    let second: Vec<u8> = vec![
        104, 105, 116, 32, 116, 104, 101, 32, 98, 117, 108, 108, 39, 115, 32, 101, 121, 101,
    ];
    let result: Result<Vec<u8>, &'static str> = hex::xor_bytes(&first, &second);

    let expected: Vec<u8> = vec![
        116, 104, 101, 32, 107, 105, 100, 32, 100, 111, 110, 39, 116, 32, 112, 108, 97, 121,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_xor_bytes_repeating() {
    let bytes: Vec<u8> = vec![
        27, 55, 55, 51, 49, 54, 63, 120, 21, 27, 127, 43, 120, 52, 49, 51, 61, 120, 57, 120, 40,
        55, 45, 54, 60, 120, 55, 62, 120, 58, 57, 59, 55, 54,
    ];
    let byte: char = 'X';
    let result: Vec<u8> = hex::xor_bytes_repeating(&bytes, byte as u8);

    let expected: Vec<u8> = vec![
        67, 111, 111, 107, 105, 110, 103, 32, 77, 67, 39, 115, 32, 108, 105, 107, 101, 32, 97, 32,
        112, 111, 117, 110, 100, 32, 111, 102, 32, 98, 97, 99, 111, 110,
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_bytes_to_hex_string() {
    let bytes: Vec<u8> = vec![
        116, 104, 101, 32, 107, 105, 100, 32, 100, 111, 110, 39, 116, 32, 112, 108, 97, 121,
    ];
    let result: String = hex::bytes_to_hex_string(&bytes);

    let expected: String = String::from("746865206B696420646F6E277420706C6179");
    assert_eq!(result, expected);
}
