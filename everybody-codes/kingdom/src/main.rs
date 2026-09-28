fn main() {
    let result = kingdom::quest_1_p3();

    if let Ok(result) = result {
        println!("Result of quest: {}", result);
    } else {
        result.unwrap();
    }
}
