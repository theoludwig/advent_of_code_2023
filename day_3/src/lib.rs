#[derive(Debug, PartialEq)]
pub struct NumberPosition {
    pub index_start: usize,
    pub index_end: usize,
    pub value: usize,
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
        });
    }
    result
}

pub fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
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
                .map(|matching_number| matching_number.value)
                .sum::<usize>()
        })
        .sum()
}

// pub fn part_2(_input: &str) -> usize {
//     42
// }

#[cfg(test)]
mod day_3_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 4361);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(include_str!("../input_example_1.txt")), 467835);
    // }
}
