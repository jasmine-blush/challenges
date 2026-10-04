pub fn quest_1_p1(input: &str) -> usize {
    let mut potions: usize = 0;
    for enemy in input.chars() {
        if enemy == 'B' {
            potions += 1;
        }
        if enemy == 'C' {
            potions += 3;
        }
    }

    potions
}

pub fn quest_1_p2(input: &str) -> usize {
    let mut potions: usize = 0;
    let mut last_enemy: Option<char> = None;
    for enemy in input.chars() {
        if let Some(pair) = last_enemy {
            if pair != 'x' && enemy != 'x' {
                potions += 2;
            }
            last_enemy = None;
        } else {
            last_enemy = Some(enemy);
        }

        if enemy == 'B' {
            potions += 1;
        } else if enemy == 'C' {
            potions += 3;
        } else if enemy == 'D' {
            potions += 5;
        }
    }

    potions
}

pub fn quest_1_p3(input: &str) -> usize {
    let mut potions: usize = 0;
    let mut group_size: usize = 0;
    let mut enemies_in_group: usize = 0;
    for enemy in input.chars() {
        if enemy == 'B' {
            potions += 1;
        } else if enemy == 'C' {
            potions += 3;
        } else if enemy == 'D' {
            potions += 5;
        }

        group_size += 1;
        if enemy != 'x' {
            enemies_in_group += 1;
        }
        if group_size == 3 {
            if enemies_in_group == 3 {
                potions += 6;
            } else if enemies_in_group == 2 {
                potions += 2;
            }
            group_size = 0;
            enemies_in_group = 0;
        }
    }

    potions
}

pub fn quest_2_p1(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let words: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .collect::<Vec<&str>>();
    let runes: &str = lines[2];

    let mut total_runes: usize = 0;
    for word in words {
        total_runes += runes.matches(word).collect::<Vec<&str>>().len();
    }

    total_runes
}

pub fn quest_2_p2(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let words: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .collect::<Vec<&str>>();

    let mut total_rune_chars: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        if i < 2 {
            continue;
        }

        let mut chars: Vec<bool> = line.chars().map(|_char| false).collect();
        for word in &words {
            for lmatch in line.match_indices(word) {
                let mut start = lmatch.0;
                let end = start + (lmatch.1.chars().count() - 1);
                while start <= end {
                    chars[start] = true;
                    start += 1;
                }
            }

            for lmatch in line.rmatch_indices(word) {
                let mut start = lmatch.0;
                let end = start + (lmatch.1.chars().count() - 1);
                while start <= end {
                    chars[start] = true;
                    start += 1;
                }
            }

            let rword: String = word.chars().rev().collect();
            for rmatch in line.match_indices(&rword) {
                let mut start = rmatch.0;
                let end = start + (rmatch.1.chars().count() - 1);

                while start <= end {
                    chars[start] = true;
                    start += 1;
                }
            }

            let rword: String = word.chars().rev().collect();
            for rmatch in line.rmatch_indices(&rword) {
                let mut start = rmatch.0;
                let end = start + (rmatch.1.chars().count() - 1);

                while start <= end {
                    chars[start] = true;
                    start += 1;
                }
            }
        }

        let chars_count: usize = chars
            .iter()
            .fold(0, |acc, e| acc + (if *e { 1 } else { 0 }));
        total_rune_chars += chars_count;
    }

    total_rune_chars
}

pub fn quest_2_p3(input: &str) -> usize {
    0
}
