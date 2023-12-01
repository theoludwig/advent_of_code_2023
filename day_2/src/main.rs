fn part_1() -> i32 {
    142
}

fn main() {
    println!("- Day 2: Title -");
    println!("Answer Part 1: {}", part_1());
}

#[cfg(test)]
mod day_2_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(142, part_1());
    }
}
