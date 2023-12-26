use std::{
    cmp::{Ordering::*, Reverse},
    iter::zip,
};

#[derive(Debug, Clone)]
pub struct Player<'h> {
    hand: Hand<'h>,
    pub bid: u32,
}

impl<'h> Player<'h> {
    pub fn new(hand_str: &'h str, bid: u32, part_2: bool) -> Self {
        Self {
            hand: Hand {
                cards: hand_str,
                part_2,
            },
            bid,
        }
    }
}

impl Ord for Player<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Player<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl Eq for Player<'_> {}

const CARDS: [char; 12] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Hand<'h> {
    cards: &'h str,
    part_2: bool,
}

impl<'h> Hand<'h> {
    #[cfg(test)]
    pub fn new(cards: &'h str, part_2: bool) -> Self {
        Self { cards, part_2 }
    }
}

impl Hand<'_> {
    fn hand_type(&self) -> HandType {
        let mut card_counts: Vec<_> = CARDS
            .iter()
            .filter_map(|card| {
                let count = self.cards.chars().filter(|c| c == card).count();

                if count > 0 {
                    Some(count)
                } else {
                    None
                }
            })
            .collect();

        let jokers = self.cards.chars().filter(|c| *c == 'J').count();

        if self.part_2 {
            card_counts.sort_unstable_by_key(|count| Reverse(*count));

            if card_counts.is_empty() {
                card_counts.push(jokers);
            } else {
                card_counts[0] += jokers;
            }
        } else {
            if jokers > 0 {
                card_counts.push(jokers);
            }
            card_counts.sort_unstable_by_key(|count| Reverse(*count));
        }

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts[0] == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts[0] == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Equal => zip(self.cards.chars(), other.cards.chars())
                .map(|(a, b)| card_value(a, self.part_2).cmp(&card_value(b, self.part_2)))
                .find(|cmp| *cmp != Equal)
                .unwrap_or(Equal),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl Eq for Hand<'_> {}

fn card_value(card: char, part_2: bool) -> u8 {
    match card {
        num @ '2'..='9' => (num as u32 - '0' as u32) as u8,
        'T' => 10,
        'J' => {
            if part_2 {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card"),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_hand_type_part_1() {
        assert_eq!(Hand::new("AAAAA", false).hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("AA8AA", false).hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("23332", false).hand_type(), HandType::FullHouse);
        assert_eq!(
            Hand::new("TTT98", false).hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(Hand::new("23432", false).hand_type(), HandType::TwoPair);
        assert_eq!(Hand::new("A23A4", false).hand_type(), HandType::OnePair);
        assert_eq!(Hand::new("23456", false).hand_type(), HandType::HighCard);
    }

    #[test]
    fn test_hand_type_part_2() {
        assert_eq!(Hand::new("32T3K", true).hand_type(), HandType::OnePair);
        assert_eq!(Hand::new("KK677", true).hand_type(), HandType::TwoPair);
        assert_eq!(Hand::new("T55J5", true).hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("KTJJT", true).hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("QQQJA", true).hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("JJJJJ", true).hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_card_value_part_2() {
        assert_eq!(card_value('J', true), 1);
        assert_eq!(card_value('2', true), 2);
        assert_eq!(card_value('9', true), 9);
        assert_eq!(card_value('T', true), 10);
    }

    #[test]
    fn test_hand_cmp_part_2() {
        assert_eq!(
            Hand::new("22222", true).cmp(&Hand::new("JJJJJ", true)),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("KTJJT", true).cmp(&Hand::new("QQQJA", true)),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("QQQJA", true).cmp(&Hand::new("T55J5", true)),
            Ordering::Greater
        );
    }
}
