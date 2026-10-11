use cryptopals::utils::hex;

fn main() {
    println!(
        "input: {:?}",
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    );
    let result = hex::hex_str_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    println!("hex: {:?}", result);
    let result = hex::bytes_to_base64(&result);
    println!("base64: {:?}", result);

    let test1 = hex::hex_str_to_bytes("1c0111001f010100061a024b53535009181c").unwrap();
    let test2 = hex::hex_str_to_bytes("686974207468652062756c6c277320657965").unwrap();
    println!("first: {:?}", test1);
    println!("second: {:?}", test2);

    let result = hex::xor_bytes(&test1, &test2).unwrap();
    println!("xor: {:?}", result);

    let result = hex::bytes_to_hex_string(&result);
    println!("result: {:?}", result);

    let result = cryptopals::get_likely_english_char_xor_message(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );
    println!("decoded: {:?}", result);
}
