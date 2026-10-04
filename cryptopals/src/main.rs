fn main() {
    println!(
        "input: {:?}",
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    );
    let result = cryptopals::hex_str_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    println!("hex: {:?}", result);
    let result = cryptopals::bytes_to_base64(&result);
    println!("base64: {:?}", result);
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::{assert_matches, debug_assert_matches};

    #[test]
    fn test_hex_char_to_byte() {
        let chars: Vec<char> = vec![
            '4', '9', '2', '7', '6', 'd', '2', '0', '6', 'b', '6', '9', '6', 'c', '6', 'c', '6',
            '9', '6', 'e', '6', '7', '2', '0', '7', '9', '6', 'f', '7', '5', '7', '2', '2', '0',
            '6', '2', '7', '2', '6', '1', '6', '9', '6', 'e', '2', '0', '6', 'c', '6', '9', '6',
            'b', '6', '5', '2', '0', '6', '1', '2', '0', '7', '0', '6', 'f', '6', '9', '7', '3',
            '6', 'f', '6', 'e', '6', 'f', '7', '5', '7', '3', '2', '0', '6', 'd', '7', '5', '7',
            '3', '6', '8', '7', '2', '6', 'f', '6', 'f', '6', 'd',
        ];
        let result: Vec<Option<u8>> = chars
            .iter()
            .map(|char| cryptopals::hex_char_to_byte(*char))
            .collect();

        let expected: Vec<u8> = vec![
            4, 9, 2, 7, 6, 13, 2, 0, 6, 11, 6, 9, 6, 12, 6, 12, 6, 9, 6, 14, 6, 7, 2, 0, 7, 9, 6,
            15, 7, 5, 7, 2, 2, 0, 6, 2, 7, 2, 6, 1, 6, 9, 6, 14, 2, 0, 6, 12, 6, 9, 6, 11, 6, 5, 2,
            0, 6, 1, 2, 0, 7, 0, 6, 15, 6, 9, 7, 3, 6, 15, 6, 14, 6, 15, 7, 5, 7, 3, 2, 0, 6, 13,
            7, 5, 7, 3, 6, 8, 7, 2, 6, 15, 6, 15, 6, 13,
        ];
        assert_matches!(result, expected);
    }

    #[test]
    fn test_parse_hex_str() {
        let result: Option<Vec<u8>> = cryptopals::hex_str_to_bytes(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        );

        let expected: Vec<u8> = vec![
            73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98,
            114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110,
            111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109,
        ];
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_bytes_to_base64() {
        let bytes: Vec<u8> = vec![
            73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98,
            114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111, 110,
            111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109,
        ];
        let result: String = cryptopals::bytes_to_base64(&bytes);

        let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(result, expected);
    }
}
