use core::str::FromStr;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Card {
    pub label: char,
}

impl Card {
    const LABELS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    pub fn strength(&self) -> usize {
        Card::LABELS.len()
            - Card::LABELS
                .iter()
                .position(|&current| current == self.label)
                .unwrap_or_default()
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Card {
    fn from(label: char) -> Self {
        Card { label }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum CardsHandType {
    #[default]
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl CardsHandType {
    pub const fn strength(&self) -> usize {
        *self as usize
    }
}

impl Ord for CardsHandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for CardsHandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CardsHand {
    pub cards: [Card; 5],
    pub bid: usize,
}

impl CardsHand {
    pub fn hand_type(&self) -> CardsHandType {
        let mut label_counts: HashMap<char, usize> = HashMap::new();
        for card in &self.cards {
            if let Some(label_count) = label_counts.get_mut(&card.label) {
                *label_count += 1;
            } else {
                label_counts.insert(card.label, 1);
            }
        }
        let keys = label_counts.keys();
        if keys.len() == 1 {
            return CardsHandType::FiveOfAKind;
        }
        if keys.len() == 2 && label_counts.iter().any(|(_, &value)| value == 4) {
            return CardsHandType::FourOfAKind;
        }
        if keys.len() == 2 && label_counts.iter().any(|(_, &value)| value == 3) {
            return CardsHandType::FullHouse;
        }
        if keys.len() == 3 && label_counts.iter().any(|(_, &value)| value == 3) {
            return CardsHandType::ThreeOfAKind;
        }
        if keys.len() == 3 && label_counts.iter().any(|(_, &value)| value == 2) {
            return CardsHandType::TwoPair;
        }
        if keys.len() == 5 {
            return CardsHandType::HighCard;
        }
        CardsHandType::OnePair
    }
}

impl Ord for CardsHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type() != other.hand_type() {
            return self
                .hand_type()
                .strength()
                .cmp(&other.hand_type().strength());
        }
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for CardsHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for CardsHand {
    type Err = &'static str;

    /// Parses a string `string` to return a value of [`CardsHand`]
    ///
    /// If parsing succeeds, return the value inside [`Ok`], otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside [`Err`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use day_7::{Card, CardsHand, CardsHandType};
    ///
    /// let string = "32T3K 765";
    /// let expected_result = CardsHand {
    ///     cards: [Card { label: '3' }, Card { label: '2' }, Card { label: 'T' }, Card { label: '3' }, Card { label: 'K' } ],
    ///     bid: 765,
    /// };
    ///
    /// let actual_result = CardsHand::from_str(string).unwrap();
    /// let actual_hand_type = actual_result.hand_type();
    /// assert_eq!(actual_result, expected_result);
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut input = string.split_ascii_whitespace();
        let result = CardsHand {
            cards: array_init::from_iter(input.next().unwrap_or_default().chars().map(Card::from))
                .unwrap_or_default(),
            bid: input
                .next()
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or_default(),
        };
        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CamelCards {
    cards_hands: Vec<CardsHand>,
}

impl FromStr for CamelCards {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(CamelCards {
            cards_hands: string
                .trim()
                .lines()
                .map(|line| CardsHand::from_str(line.trim()).unwrap_or_default())
                .collect::<Vec<CardsHand>>(),
        })
    }
}

pub fn part_1(input: &str) -> usize {
    let mut camel_cards = CamelCards::from_str(input).unwrap_or_default();
    camel_cards.cards_hands.sort();
    camel_cards
        .cards_hands
        .iter()
        .enumerate()
        .map(|(index, card_hand)| {
            let rank = index + 1;
            card_hand.bid * rank
        })
        .sum()
}

#[cfg(test)]
mod day_7_tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("../input_example_1.txt")), 6440);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 250370104);
    }

    mod hand_types {
        use std::str::FromStr;

        use crate::{CardsHand, CardsHandType};

        #[test]
        fn test_five_of_a_kind() {
            let cards_hand = CardsHand::from_str("AAAAA").unwrap();
            let expected = CardsHandType::FiveOfAKind;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_four_of_a_kind() {
            let cards_hand = CardsHand::from_str("AA8AA").unwrap();
            let expected = CardsHandType::FourOfAKind;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_full_house() {
            let cards_hand = CardsHand::from_str("23332").unwrap();
            let expected = CardsHandType::FullHouse;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_three_of_a_kind() {
            let cards_hand = CardsHand::from_str("TTT98").unwrap();
            let expected = CardsHandType::ThreeOfAKind;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_two_pair() {
            let cards_hand = CardsHand::from_str("23432").unwrap();
            let expected = CardsHandType::TwoPair;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_one_pair() {
            let cards_hand = CardsHand::from_str("A23A4").unwrap();
            let expected = CardsHandType::OnePair;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_high_card() {
            let cards_hand = CardsHand::from_str("23456").unwrap();
            let expected = CardsHandType::HighCard;
            let actual = cards_hand.hand_type();
            assert_eq!(actual, expected);
        }
    }
}
