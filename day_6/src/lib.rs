use rayon::prelude::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Race {
    pub maximum_time_in_milliseconds: usize,
    pub best_distance_in_millimeters_recorded: usize,
}

impl Race {
    pub fn get_all_winning_strategies(&self) -> Vec<RaceStrategy> {
        (1..self.maximum_time_in_milliseconds)
            .into_par_iter()
            .map(|time_in_milliseconds_holding_button| RaceStrategy {
                time_in_milliseconds_holding_button,
                maximum_time_in_milliseconds: self.maximum_time_in_milliseconds,
            })
            .filter(|race_strategy| {
                let distance_in_millimeters_travelled =
                    race_strategy.calculate_distance_in_millimeters_travelled();
                distance_in_millimeters_travelled > self.best_distance_in_millimeters_recorded
            })
            .collect::<Vec<RaceStrategy>>()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RaceStrategy {
    pub time_in_milliseconds_holding_button: usize,
    pub maximum_time_in_milliseconds: usize,
}

impl RaceStrategy {
    pub fn calculate_distance_in_millimeters_travelled(&self) -> usize {
        (self.maximum_time_in_milliseconds - self.time_in_milliseconds_holding_button)
            * self.time_in_milliseconds_holding_button
    }
}

pub fn part_1(input: &str) -> usize {
    fn get_numbers_from_line(line: &str) -> Vec<usize> {
        line.split(':')
            .last()
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| {
                let result: usize = value.parse().unwrap_or_default();
                result
            })
            .collect::<Vec<usize>>()
    }

    let mut lines = input.trim().lines();
    let maximum_times = get_numbers_from_line(lines.next().unwrap_or_default());
    let best_distances = get_numbers_from_line(lines.next().unwrap_or_default());
    maximum_times
        .iter()
        .zip(best_distances.iter())
        .map(
            |(&maximum_time_in_milliseconds, &best_distance_recorded_in_millimeters)| Race {
                maximum_time_in_milliseconds,
                best_distance_in_millimeters_recorded: best_distance_recorded_in_millimeters,
            },
        )
        .map(|race| race.get_all_winning_strategies().len())
        .product()
}

pub fn part_2(input: &str) -> usize {
    fn get_number_from_line(line: &str) -> usize {
        line.split(':')
            .last()
            .unwrap_or_default()
            .replace(' ', "")
            .trim()
            .parse()
            .unwrap_or_default()
    }

    let mut lines = input.trim().lines();
    let race = Race {
        maximum_time_in_milliseconds: get_number_from_line(lines.next().unwrap_or_default()),
        best_distance_in_millimeters_recorded: get_number_from_line(
            lines.next().unwrap_or_default(),
        ),
    };
    race.get_all_winning_strategies().len()
}

#[cfg(test)]
mod day_6_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 288);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 71503);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 1083852);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 23501589);
    }
}
