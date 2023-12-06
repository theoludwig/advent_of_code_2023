use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct CardNumbers {
    pub numbers: Vec<u32>,
}

impl FromStr for CardNumbers {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`CardNumbers`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use day_4::CardNumbers;
    ///
    /// let string = "83 86  6 31 17  9 48 53";
    /// let expected_result = CardNumbers {
    ///     numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
    /// };
    /// let actual_result = CardNumbers::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let result = CardNumbers {
            numbers: string
                .split_whitespace()
                .map(|string| string.trim())
                .filter(|&string| !string.is_empty())
                .map(|string| string.parse::<u32>().unwrap_or_default())
                .collect(),
        };
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Card {
    pub id: usize,
    pub winning_numbers: CardNumbers,
    pub owned_numbers: CardNumbers,
}

impl FromStr for Card {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`Card`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use day_4::{Card, CardNumbers};
    ///
    /// let string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    /// let expected_result = Card {
    ///     id: 1,
    ///     winning_numbers: CardNumbers { numbers: vec![41, 48, 83, 86, 17], },
    ///     owned_numbers: CardNumbers { numbers: vec![83, 86, 6, 31, 17, 9, 48, 53], },
    /// };
    /// let actual_result = Card::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result = Card::default();
        let mut parts = string.split(": ");
        result.id = parts
            .next()
            .unwrap_or("Card 1")
            .strip_prefix("Card ")
            .unwrap_or("1")
            .parse()
            .unwrap_or(1);

        let mut numbers_parts = parts.next().unwrap_or("").split(" | ");

        result.winning_numbers =
            CardNumbers::from_str(numbers_parts.next().unwrap_or("")).unwrap_or_default();
        result.owned_numbers =
            CardNumbers::from_str(numbers_parts.next().unwrap_or("")).unwrap_or_default();

        Ok(result)
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap_or_default())
        .map(|card| {
            let numbers_matches_count = card
                .owned_numbers
                .numbers
                .iter()
                .filter(|&owned_number| card.winning_numbers.numbers.contains(owned_number))
                .count() as u32;
            let base: usize = 2;
            if numbers_matches_count > 0 {
                base.pow(numbers_matches_count.saturating_sub(1))
            } else {
                0
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_4_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 13);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(include_str!("../input_example_1.txt")), 467835);
    // }
}
