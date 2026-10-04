fn main() {
    let input: String = std::fs::read_to_string("./assets/quest_2_p2.txt").unwrap();
    let result = kingdom::quest_2_p2(&input);

    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_1_p1() {
        let input: String = std::fs::read_to_string("./assets/quest_1_p1.txt").unwrap();
        let result = kingdom::quest_1_p1(&input);

        assert_eq!(result, 1397);
    }

    #[test]
    fn quest_1_p2() {
        let input: String = std::fs::read_to_string("./assets/quest_1_p2.txt").unwrap();
        let result = kingdom::quest_1_p2(&input);

        assert_eq!(result, 5388);
    }

    #[test]
    fn quest_1_p3() {
        let input: String = std::fs::read_to_string("./assets/quest_1_p3.txt").unwrap();
        let result = kingdom::quest_1_p3(&input);

        assert_eq!(result, 27482);
    }

    #[test]
    fn quest_2_p1() {
        let input: String = std::fs::read_to_string("./assets/quest_2_p1.txt").unwrap();
        let result = kingdom::quest_2_p1(&input);

        assert_eq!(result, 29);
    }

    #[test]
    fn quest_2_p2() {
        let input: String = std::fs::read_to_string("./assets/quest_2_p2.txt").unwrap();
        let result = kingdom::quest_2_p2(&input);

        assert_eq!(result, 5119);
    }

    #[test]
    fn quest_2_p3() {
        let input: String = std::fs::read_to_string("./assets/quest_2_p3.txt").unwrap();
        let result = kingdom::quest_2_p3(&input);

        assert_eq!(result, usize::MAX);
    }
}
