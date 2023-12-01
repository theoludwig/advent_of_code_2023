fn part_1(input: &str) -> i32 {
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
            let number: i32 = number.parse().expect("Should parse as a number.");
            number
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
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
            let number: i32 = number.parse().expect("Should parse as a number.");
            number
        })
        .sum()
}

fn main() {
    let input_example1 = include_str!("../input_example_1.txt");
    let input_example2 = include_str!("../input_example_2.txt");
    let input = include_str!("../input.txt");

    println!("Answer Part 1 (example1): {}", part_1(input_example1));
    println!("Answer Part 1 (example2): {}", part_1(input_example2));
    println!("Answer Part 1: {}", part_1(input));

    println!("Answer Part 2 (example1): {}", part_2(input_example1));
    println!("Answer Part 2 (example2): {}", part_2(input_example2));
    println!("Answer Part 2: {}", part_2(input));
}
