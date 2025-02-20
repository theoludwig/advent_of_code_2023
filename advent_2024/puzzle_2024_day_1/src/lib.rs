use std::collections::HashMap;

pub fn part_1(input: &str) -> i32 {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let mut values = line.split_whitespace();
        let left = values
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or_default();
        let right = values
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or_default();
        left_list.push(left);
        right_list.push(right);
    });
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let mut values = line.split_whitespace();
        let left = values
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or_default();
        let right = values
            .next()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or_default();
        left_list.push(left);
        right_list.push(right);
    });

    let mut right_apparition: HashMap<i32, i32> = HashMap::new();
    for value in right_list {
        right_apparition
            .entry(value)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    left_list
        .iter()
        .map(|value| value * right_apparition.get(value).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod puzzle_2024_day_1_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 11);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 31);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 2904518);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 18650129);
    }
}
