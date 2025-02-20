use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RangeConverter {
    pub source_range: Range<usize>,
    pub destination_range: Range<usize>,
}

impl FromStr for RangeConverter {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`RangeConverter`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use puzzle_2023_day_5::RangeConverter;
    ///
    /// let string = "50 98 2 ";
    /// let expected_result = RangeConverter {
    ///     source_range: 98..100,
    ///     destination_range: 50..52,
    /// };
    /// let actual_result = RangeConverter::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let numbers = string
            .trim()
            .split_ascii_whitespace()
            .map(|string| {
                let result: usize = string.trim().parse().unwrap_or_default();
                result
            })
            .collect::<Vec<usize>>();
        let destination_range_start = *numbers.first().unwrap_or(&0);
        let source_range_start = *numbers.get(1).unwrap_or(&0);
        let range_length = *numbers.get(2).unwrap_or(&0);
        let result = RangeConverter {
            source_range: source_range_start..(source_range_start + range_length),
            destination_range: destination_range_start..(destination_range_start + range_length),
        };
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CategoryConverter {
    pub name: String,
    pub ranges_converters: Vec<RangeConverter>,
}

impl FromStr for CategoryConverter {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`CategoryConverter`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use puzzle_2023_day_5::{CategoryConverter, RangeConverter};
    ///
    /// let string = "
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    /// ";
    /// let expected_result = CategoryConverter {
    ///     name: String::from("seed-to-soil map:"),
    ///     ranges_converters: vec![
    ///         RangeConverter {
    ///             source_range: 98..100,
    ///             destination_range: 50..52,
    ///         },
    ///         RangeConverter {
    ///             source_range: 50..98,
    ///             destination_range: 52..100,
    ///         },
    ///     ],
    /// };
    /// let actual_result = CategoryConverter::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut lines = string.trim().lines();
        let name = lines.next().unwrap_or_default();
        let mut ranges_converters = vec![];
        for line in lines {
            ranges_converters.push(RangeConverter::from_str(line).unwrap_or_default());
        }
        let result = CategoryConverter {
            name: String::from(name),
            ranges_converters,
        };
        Ok(result)
    }
}

impl CategoryConverter {
    pub fn convert(&self, number: usize) -> usize {
        self.ranges_converters
            .iter()
            .find_map(|range_converter| {
                if range_converter.source_range.contains(&number) {
                    Some(
                        range_converter.destination_range.start
                            + (number - range_converter.source_range.start),
                    )
                } else {
                    None
                }
            })
            .unwrap_or(number)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub categories_converters: Vec<CategoryConverter>,
}

impl FromStr for Almanac {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`Almanac`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use puzzle_2023_day_5::{Almanac, CategoryConverter, RangeConverter};
    ///
    /// let string = "
    /// seeds: 79 14 55 13
    ///
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    /// ";
    /// let expected_result = Almanac {
    ///     seeds: vec![79, 14, 55, 13],
    ///     categories_converters: vec![CategoryConverter {
    ///         name: String::from("seed-to-soil map:"),
    ///         ranges_converters: vec![
    ///             RangeConverter {
    ///                 source_range: 98..100,
    ///                 destination_range: 50..52,
    ///             },
    ///             RangeConverter {
    ///                 source_range: 50..98,
    ///                 destination_range: 52..100,
    ///             },
    ///         ],
    ///     }],
    /// };
    /// let actual_result = Almanac::from_str(string).unwrap();
    ///
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut categories = string.trim().split("\n\n");
        let seeds = categories
            .next()
            .unwrap_or_default()
            .strip_prefix("seeds: ")
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(|string| {
                let result: usize = string.trim().parse().unwrap_or_default();
                result
            })
            .collect::<Vec<usize>>();
        let categories_converters = categories
            .map(|category_string| CategoryConverter::from_str(category_string).unwrap_or_default())
            .collect::<Vec<CategoryConverter>>();
        let result = Almanac {
            seeds,
            categories_converters,
        };
        Ok(result)
    }
}

impl Almanac {
    pub fn minimum_location(&self) -> usize {
        self.seeds
            .par_iter()
            .map(|&seed| {
                self.categories_converters
                    .iter()
                    .fold(seed, |accumulator, category_converter| {
                        category_converter.convert(accumulator)
                    })
            })
            .progress()
            .min()
            .unwrap_or_default()
    }

    pub fn set_seeds_as_range_pairs(&mut self) {
        self.seeds = self
            .seeds
            .par_chunks(2)
            .progress()
            .flat_map(|chunk| {
                if let [range_start, range_length] = chunk {
                    let start = *range_start;
                    let length = *range_length;
                    let range = start..(start + length);
                    range.into_iter()
                } else {
                    let empty_range = 0..0;
                    empty_range.into_iter()
                }
            })
            .collect();
    }
}

pub fn part_1(input: &str) -> usize {
    let almanac = Almanac::from_str(input).unwrap_or_default();
    almanac.minimum_location()
}

pub fn part_2(input: &str) -> usize {
    let mut almanac = Almanac::from_str(input).unwrap_or_default();
    almanac.set_seeds_as_range_pairs();
    almanac.minimum_location()
}

#[cfg(test)]
mod puzzle_2023_day_5_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 35);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("../input_example_1.txt")), 46);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 313045984);
    }

    #[test]
    #[ignore]
    /// Ignored because it is a expensive/slow test to run.
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 20283860);
    }
}
