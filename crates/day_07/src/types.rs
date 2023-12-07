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
    pub fn new(hand_str: &'h str, bid: u32) -> Self {
        Self {
            hand: Hand { cards: hand_str },
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

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

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
}

impl<'h> Hand<'h> {
    #[cfg(test)]
    pub fn new(cards: &'h str) -> Self {
        Self { cards }
    }
}

impl Hand<'_> {
    fn hand_type(&self) -> HandType {
        let mut card_counts: Vec<_> = CARDS
            .iter()
            .filter_map(|card| {
                let count = self.cards.chars().filter(|c| c == card).count();

                if count > 0 {
                    Some((card, count))
                } else {
                    None
                }
            })
            .collect();

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                card_counts.sort_unstable_by_key(|(_, count)| Reverse(*count));

                if card_counts[0].1 == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                card_counts.sort_unstable_by_key(|(_, count)| Reverse(*count));

                if card_counts[0].1 == 3 {
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
                .map(|(a, b)| card_value(a).cmp(&card_value(b)))
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

fn card_value(card: char) -> u8 {
    match card {
        num @ '2'..='9' => (num as u32 - '2' as u32) as u8,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid card"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand::new("AAAAA").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("AA8AA").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("23332").hand_type(), HandType::FullHouse);
        assert_eq!(Hand::new("TTT98").hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::new("23432").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::new("A23A4").hand_type(), HandType::OnePair);
        assert_eq!(Hand::new("23456").hand_type(), HandType::HighCard);
    }

    #[test]
    fn test_hand_compare() {
        assert_eq!(Hand::new("AAAAA").hand_type(), HandType::FiveOfAKind);
    }
}
