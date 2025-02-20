pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let characters_digits = line
                .chars()
                .filter(|&character| character.is_ascii_digit())
                .collect::<Vec<char>>();

            let first_digit = characters_digits.first().unwrap_or(&'0').to_owned();
            let last_digit = characters_digits.last().unwrap_or(&'0').to_owned();
            let number = format!("{}{}", first_digit, last_digit);
            let number: usize = number.parse().expect("Should parse as a `usize`.");
            number
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let numbers_spelled_out_with_letters = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let mut characters_digits: Vec<char> = vec![];
            let mut temporary = String::from("");
            for character in line.chars() {
                temporary += &character.to_string();

                let mut temporary_spelled_number_index = None;
                for (index, spelled_number) in numbers_spelled_out_with_letters.iter().enumerate() {
                    if temporary.contains(spelled_number) {
                        temporary_spelled_number_index = Some(index);
                        break;
                    }
                }
                if let Some(temporary_spelled_number_index) = temporary_spelled_number_index {
                    let number = temporary_spelled_number_index + 1;
                    characters_digits.push(
                        number
                            .to_string()
                            .chars()
                            .next()
                            .expect("Number should be single-character digit."),
                    );
                    temporary = character.to_string();
                }

                if character.is_ascii_digit() {
                    characters_digits.push(character);
                    temporary = String::from("");
                }
            }

            let first_digit = characters_digits.first().unwrap_or(&'0').to_owned();
            let last_digit = characters_digits.last().unwrap_or(&'0').to_owned();
            let number = format!("{}{}", first_digit, last_digit);
            let number: usize = number.parse().expect("Should parse as a `usize`.");
            number
        })
        .sum()
}

#[cfg(test)]
mod puzzle_2023_day_1_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 142);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_2.txt")), 281);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 55130);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 54985);
    }
}
