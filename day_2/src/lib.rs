use std::str::FromStr;

const NUMBER_OF_RED_CUBES_IN_THE_BAG: usize = 12;
const NUMBER_OF_GREEN_CUBES_IN_THE_BAG: usize = 13;
const NUMBER_OF_BLUE_CUBES_IN_THE_BAG: usize = 14;

#[derive(Debug, Default, PartialEq)]
pub struct NumberOfCubesOfEachColor {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl FromStr for NumberOfCubesOfEachColor {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`NumberOfCubesOfEachColor`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use day_2::NumberOfCubesOfEachColor;
    ///
    /// let string = "3 blue, 4 red";
    /// let expected_result = NumberOfCubesOfEachColor {
    ///     red: 4,
    ///     green: 0,
    ///     blue: 3,
    /// };
    /// let actual_result = NumberOfCubesOfEachColor::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result = NumberOfCubesOfEachColor::default();
        let subsets = string.split(", ");
        for subset in subsets {
            let mut subset_splitted = subset.split_ascii_whitespace();
            let number: usize = subset_splitted.next().unwrap_or("0").parse().unwrap_or(0);
            let color = subset_splitted.next();
            if let Some(color) = color {
                match color {
                    "red" => result.red = number,
                    "blue" => result.blue = number,
                    "green" => result.green = number,
                    _ => {}
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Game {
    pub id: usize,
    pub subsets_of_cubes: Vec<NumberOfCubesOfEachColor>,
}

impl FromStr for Game {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`Game`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use day_2::{Game, NumberOfCubesOfEachColor};
    ///
    /// let string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    /// let expected_result = Game {
    ///     id: 1,
    ///     subsets_of_cubes: vec![
    ///         NumberOfCubesOfEachColor {
    ///             red: 4,
    ///             green: 0,
    ///             blue: 3,
    ///         },
    ///         NumberOfCubesOfEachColor {
    ///             red: 1,
    ///             green: 2,
    ///             blue: 6,
    ///         },
    ///         NumberOfCubesOfEachColor {
    ///             red: 0,
    ///             green: 2,
    ///             blue: 0,
    ///         },
    ///     ],
    /// };
    /// let actual_result = Game::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result = Game::default();
        let mut parts = string.split(": ");
        result.id = parts
            .next()
            .unwrap_or("Game 1")
            .strip_prefix("Game ")
            .unwrap_or("1")
            .parse()
            .unwrap_or(1);
        result.subsets_of_cubes = parts
            .next()
            .unwrap_or("0 red, 0 green, 0 blue")
            .split("; ")
            .map(|string| {
                NumberOfCubesOfEachColor::from_str(string)
                    .unwrap_or(NumberOfCubesOfEachColor::default())
            })
            .collect::<Vec<NumberOfCubesOfEachColor>>();
        Ok(result)
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap_or_default())
        .filter(|game| {
            game.subsets_of_cubes
                .iter()
                .all(|number_of_cubes_of_each_color| {
                    number_of_cubes_of_each_color.red <= NUMBER_OF_RED_CUBES_IN_THE_BAG
                        && number_of_cubes_of_each_color.blue <= NUMBER_OF_BLUE_CUBES_IN_THE_BAG
                        && number_of_cubes_of_each_color.green <= NUMBER_OF_GREEN_CUBES_IN_THE_BAG
                })
        })
        .map(|game| game.id)
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap_or_default();
            let mut maximum_number_of_cubes_of_each_color = NumberOfCubesOfEachColor {
                red: 1,
                green: 1,
                blue: 1,
            };
            for number_of_cubes_of_each_color in game.subsets_of_cubes {
                if number_of_cubes_of_each_color.red > maximum_number_of_cubes_of_each_color.red {
                    maximum_number_of_cubes_of_each_color.red = number_of_cubes_of_each_color.red;
                }
                if number_of_cubes_of_each_color.green > maximum_number_of_cubes_of_each_color.green
                {
                    maximum_number_of_cubes_of_each_color.green =
                        number_of_cubes_of_each_color.green;
                }
                if number_of_cubes_of_each_color.blue > maximum_number_of_cubes_of_each_color.blue {
                    maximum_number_of_cubes_of_each_color.blue = number_of_cubes_of_each_color.blue;
                }
            }
            maximum_number_of_cubes_of_each_color.red
                * maximum_number_of_cubes_of_each_color.green
                * maximum_number_of_cubes_of_each_color.blue
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_2_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 8);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 2286);
    }
}
