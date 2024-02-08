use core::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DesertMap {
    pub directions: Vec<HorizontalDirection>,
    pub nodes: HashMap<String, (String, String)>,
}

impl FromStr for DesertMap {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`DesertMap`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use std::collections::HashMap;
    /// use day_8::{DesertMap, HorizontalDirection};
    ///
    /// let string = "
    /// RL
    ///
    /// AAA = (BBB, CCC)
    /// BBB = (DDD, EEE)
    /// CCC = (ZZZ, GGG)
    /// ";
    /// let expected_result = DesertMap {
    ///     directions: vec![HorizontalDirection::Right, HorizontalDirection::Left],
    ///     nodes: HashMap::from([
    ///         (String::from("AAA"), (String::from("BBB"), String::from("CCC"))),
    ///         (String::from("BBB"), (String::from("DDD"), String::from("EEE"))),
    ///         (String::from("CCC"), (String::from("ZZZ"), String::from("GGG"))),
    ///    ])
    /// };
    ///
    /// let actual_result = DesertMap::from_str(string).unwrap();
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result = DesertMap::default();
        let mut lines = string.trim().lines();
        let first_line = lines.next().unwrap_or_default();
        for character in first_line.chars() {
            result.directions.push(HorizontalDirection::from(character));
        }
        lines.next();
        for line in lines {
            let mut line_splitted = line.split(" = ");
            let key = line_splitted.next().unwrap_or_default();
            let values_line = line_splitted
                .next()
                .unwrap_or_default()
                .replace(['(', ')'], "");
            let mut values_line = values_line.split(", ");
            let value = (
                values_line.next().unwrap_or_default().to_string(),
                values_line.next().unwrap_or_default().to_string(),
            );
            result.nodes.insert(key.to_string(), value);
        }
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum HorizontalDirection {
    #[default]
    Left,
    Right,
}

impl From<char> for HorizontalDirection {
    fn from(direction: char) -> Self {
        if direction == 'R' {
            return HorizontalDirection::Right;
        }
        HorizontalDirection::Left
    }
}

const KEY_START: &str = "AAA";

const KEY_END: &str = "ZZZ";

pub fn part_1(input: &str) -> usize {
    let desert_map = DesertMap::from_str(input).unwrap_or_default();
    let mut steps = 0;
    let mut current_step_key = KEY_START.to_string();
    let mut current_direction_index: usize = 0;

    while current_step_key != KEY_END {
        let current_direction = desert_map
            .directions
            .get(current_direction_index)
            .cloned()
            .unwrap_or_else(|| {
                current_direction_index = 0;
                desert_map
                    .directions
                    .get(current_direction_index)
                    .cloned()
                    .unwrap_or_default()
            });

        let current_step_value = desert_map
            .nodes
            .get(&current_step_key)
            .cloned()
            .unwrap_or_default();
        current_step_key = match current_direction {
            HorizontalDirection::Left => current_step_value.0.clone(),
            HorizontalDirection::Right => current_step_value.1.clone(),
        };

        current_direction_index += 1;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod day_8_tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 2);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(include_str!("../input_example_2.txt")), 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 15871);
    }
}
