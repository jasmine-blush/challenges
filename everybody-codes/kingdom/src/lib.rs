use std::error::Error;

pub fn quest_1_p1() -> Result<usize, Box<dyn Error>> {
    let enemies: String = std::fs::read_to_string("./assets/quest_1_p1.txt")?;

    let mut potions: usize = 0;
    for enemy in enemies.chars() {
        if enemy == 'B' {
            potions += 1;
        }
        if enemy == 'C' {
            potions += 3;
        }
    }
    Ok(potions)
}

pub fn quest_1_p2() -> Result<usize, Box<dyn Error>> {
    let enemies: String = std::fs::read_to_string("./assets/quest_1_p2.txt")?;

    let mut potions: usize = 0;
    let mut last_enemy: Option<char> = None;
    for enemy in enemies.chars() {
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

    Ok(potions)
}

pub fn quest_1_p3() -> Result<usize, Box<dyn Error>> {
    let enemies: String = std::fs::read_to_string("./assets/quest_1_p3.txt")?;

    let mut potions: usize = 0;
    let mut group_size: usize = 0;
    let mut enemies_in_group: usize = 0;
    for enemy in enemies.chars() {
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

    Ok(potions)
}
