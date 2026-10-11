use crate::utils::hex;

pub mod utils;

const ENGLISH_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const ENGLISH_CHAR_FREQUENCIES: [f32; 26] = [
    8.2, 1.5, 2.8, 4.3, 12.7, 2.2, 2.0, 6.1, 7.0, 0.16, 0.77, 4.0, 2.4, 6.7, 7.5, 1.9, 0.12, 6.0,
    6.3, 9.1, 2.8, 0.98, 2.4, 0.15, 2.0, 0.074,
];

/// XORs the passed message against all english upper-case letters.
/// Whichever result comes closest to a natural english letter distribution is assumed correct.
/// The "correct" result string is returned together with the XOR char used.
pub fn get_likely_english_char_xor_message(message: &str) -> (String, char) {
    let mut result: (Vec<u8>, char) = (Vec::new(), '_');

    let bytes = hex::hex_str_to_bytes(message).unwrap();
    let mut best_freq: f32 = -1.0;
    for char in &ENGLISH_CHARS {
        let message_candidate: Vec<u8> = hex::xor_bytes_repeating(&bytes, *char as u8);

        let mut letter_counts: [usize; 26] = [0; 26];
        let mut letter_sum: usize = 0;
        for byte in &message_candidate {
            if let Some(letter) = char::from_u32(*byte as u32) {
                let pos = ENGLISH_CHARS
                    .iter()
                    .position(|c| *c == letter.to_ascii_uppercase());
                if let Some(idx) = pos {
                    letter_counts[idx] += 1;
                    letter_sum += 1;
                }
            }
        }

        let mut english_frequencies_delta: f32 = 0.0;
        for (j, count) in letter_counts.iter().enumerate() {
            let frequency = (*count as f32 / letter_sum as f32) * 100.0;
            let delta = frequency - ENGLISH_CHAR_FREQUENCIES[j];
            english_frequencies_delta += delta.abs();
        }

        if english_frequencies_delta < best_freq || best_freq < 0.0 {
            result = (message_candidate, *char);
            best_freq = english_frequencies_delta;
        }
    }

    (hex::bytes_to_char_string(&result.0), result.1)
}
