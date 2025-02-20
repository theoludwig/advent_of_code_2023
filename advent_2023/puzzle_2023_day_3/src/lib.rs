use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct GearPosition {
    pub index_line: usize,
    pub index_character: usize,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct NumberPosition {
    pub index_start: usize,
    pub index_end: usize,
    pub value: usize,
    pub gear_position: GearPosition,
}

pub fn get_numbers_positions_from_line(line: &str) -> Vec<NumberPosition> {
    let mut result = vec![];
    let mut number_string = String::from("");
    let mut index_start = 0;
    let mut index_end = 0;
    for (index_character, character) in line.chars().enumerate() {
        if character.is_ascii_digit() {
            if number_string.is_empty() {
                index_start = index_character;
            }
            index_end = index_character;
            number_string += &character.to_string();
        } else if !number_string.is_empty() {
            let value: usize = number_string.parse().expect("Should parse as a `usize`");
            result.push(NumberPosition {
                index_start,
                index_end,
                value,
                gear_position: GearPosition::default(),
            });
            number_string = String::from("");
            index_start = 0;
            index_end = 0;
        }
    }
    if !number_string.is_empty() {
        let value: usize = number_string.parse().expect("Should parse as a `usize`");
        result.push(NumberPosition {
            index_start,
            index_end,
            value,
            gear_position: GearPosition::default(),
        });
    }
    result
}

pub fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
}

pub fn is_gear_symbol(character: char) -> bool {
    character == '*'
}

pub fn part_1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .enumerate()
        .map(|(index_line, &line)| {
            get_numbers_positions_from_line(line)
                .iter()
                .filter(|&number_position| {
                    let index_start = number_position.index_start.saturating_sub(1);
                    let index_end = if number_position.index_end + 1 >= line.len() {
                        line.len().saturating_sub(1)
                    } else {
                        number_position.index_end + 1
                    };

                    let has_symbol_on_the_left =
                        line.chars().nth(index_start).is_some_and(is_symbol);
                    let has_symbol_on_the_right =
                        line.chars().nth(index_end).is_some_and(is_symbol);

                    let has_symbol_on_the_top = lines
                        .get(index_line.saturating_sub(1))
                        .is_some_and(|&line| {
                            line.get(index_start..=index_end)
                                .is_some_and(|value| value.chars().any(is_symbol))
                        });

                    let has_symbol_on_the_bottom = lines.get(index_line + 1).is_some_and(|&line| {
                        line.get(index_start..=index_end)
                            .is_some_and(|value| value.chars().any(is_symbol))
                    });

                    has_symbol_on_the_left
                        || has_symbol_on_the_right
                        || has_symbol_on_the_top
                        || has_symbol_on_the_bottom
                })
                .map(|number_position| number_position.value)
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut number_positions = vec![];

    lines.iter().enumerate().for_each(|(index_line, &line)| {
        get_numbers_positions_from_line(line)
            .iter()
            .for_each(|&number_position| {
                let index_start = number_position.index_start.saturating_sub(1);
                let index_end = if number_position.index_end + 1 >= line.len() {
                    line.len().saturating_sub(1)
                } else {
                    number_position.index_end + 1
                };

                let has_symbol_on_the_left =
                    line.chars().nth(index_start).is_some_and(is_gear_symbol);
                if has_symbol_on_the_left {
                    let mut number_position = number_position.to_owned();
                    number_position.gear_position.index_line = index_line;
                    number_position.gear_position.index_character = index_start;
                    number_positions.push(number_position);
                }

                let has_symbol_on_the_right =
                    line.chars().nth(index_end).is_some_and(is_gear_symbol);
                if has_symbol_on_the_right {
                    let mut number_position = number_position.to_owned();
                    number_position.gear_position.index_line = index_line;
                    number_position.gear_position.index_character = index_end;
                    number_positions.push(number_position);
                }

                let index_character_top =
                    lines.get(index_line.saturating_sub(1)).and_then(|&line| {
                        line.get(index_start..=index_end)
                            .and_then(|value| value.chars().position(is_gear_symbol))
                    });
                if let Some(index_character_top) = index_character_top {
                    let mut number_position = number_position.to_owned();
                    number_position.gear_position.index_line = index_line.saturating_sub(1);
                    number_position.gear_position.index_character =
                        index_character_top + index_start;
                    number_positions.push(number_position);
                }

                let index_character_bottom = lines.get(index_line + 1).and_then(|&line| {
                    line.get(index_start..=index_end)
                        .and_then(|value| value.chars().position(is_gear_symbol))
                });
                if let Some(index_character_bottom) = index_character_bottom {
                    let mut number_position = number_position.to_owned();
                    number_position.gear_position.index_line = index_line + 1;
                    number_position.gear_position.index_character =
                        index_character_bottom + index_start;
                    number_positions.push(number_position);
                }
            });
    });

    // Key: "index_line-index_character"
    // Value: usize
    let mut gear_positions: HashMap<String, Vec<usize>> = HashMap::new();

    number_positions.iter().for_each(|&number_position| {
        let key = format!(
            "{}-{}",
            number_position.gear_position.index_line, number_position.gear_position.index_character,
        );
        match gear_positions.get_mut(&key) {
            Some(gear_positions) => {
                gear_positions.push(number_position.value);
            }
            None => {
                gear_positions.insert(key, vec![number_position.value]);
            }
        }
    });

    let mut result = 0;

    for (_, value) in gear_positions.iter() {
        if value.len() != 2 {
            continue;
        }

        let first_number = value.first();
        let second_number = value.last();

        if let (Some(first_number), Some(second_number)) = (first_number, second_number) {
            result += first_number * second_number;
        }
    }

    result
}

#[cfg(test)]
mod puzzle_2023_day_3_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 4361);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 467835);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 553079);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 84363105);
    }
}
