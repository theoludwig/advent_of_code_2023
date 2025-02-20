use std::cmp::Ordering;

pub fn is_safe_levels(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let mut previous_level: Option<i32> = None;
    let mut previous_direction: Option<Direction> = None;
    for level in levels {
        if let Some(previous_level_value) = previous_level {
            let direction = {
                match previous_level_value.cmp(level) {
                    Ordering::Greater => Direction::Decreasing,
                    Ordering::Less => Direction::Increasing,
                    Ordering::Equal => Direction::Equal,
                }
            };
            let distance: i32 = level - previous_level_value;
            let distance = distance.abs();
            if !(1..=3).contains(&distance) {
                return false;
            }
            if let Some(previous_direction_value) = previous_direction {
                if previous_direction_value != direction || direction == Direction::Equal {
                    return false;
                }
            }
            previous_direction = Some(direction);
        }
        previous_level = Some(*level);
    }
    true
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    Equal,
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>();
            is_safe_levels(&levels)
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>();
            let is_safe = is_safe_levels(&levels);
            if is_safe {
                true
            } else {
                for index in 0..levels.len() {
                    let mut levels_cloned = levels.clone();
                    levels_cloned.remove(index);
                    if is_safe_levels(&levels_cloned) {
                        return true;
                    }
                }
                false
            }
        })
        .count()
}

#[cfg(test)]
mod puzzle_2024_day_2_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 2);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 483);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 528);
    }
}
