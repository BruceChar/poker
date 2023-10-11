use core::panic;
use std::{
    collections::HashMap,
    env::consts::FAMILY,
    fmt::{Display, Formatter},
};

use crate::{
    card::{Card, Value},
    error::Error,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct HoldemHand([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    Pair,
    TwoPair,
    Set,
    Straight,
    Flush,
    FullHouse,
    Bomb,
    StraightFlush,
    RoyalStraightFlush,
}

impl HoldemHand {
    fn new(mut cards: [Card; 5]) -> Self {
        cards.sort_by(|a, b| b.value().cmp(&a.value()));
        Self(cards)
    }

    fn rank(&self) -> Rank {
        let mut counts = HashMap::with_capacity(5);

        let cards = self.0;

        let mut is_flush = true;
        let mut is_straight = true;
        let mut pre = cards[0];
        counts.insert(pre.value(), 1);

        for cur in &cards[1..] {
            if is_flush && cur.suit() != pre.suit() {
                is_flush = false;
            }
            if is_straight && cur.value() + 1 != pre.value() {
                // "As 5c 4d 3h 2s"
                if !(pre.value() == Value::Ace && cur.value() == Value::Five) {
                    is_straight = false;
                }
            }
            *counts.entry(cur.value()).or_insert(0) += 1;
            pre = *cur;
        }
        match counts.len() {
            5 => match (is_flush, is_straight) {
                (true, true) => {
                    if cards[1].value() == Value::King {
                        return Rank::RoyalStraightFlush;
                    }
                    return Rank::StraightFlush;
                }
                (true, false) => return Rank::Flush,
                (false, true) => return Rank::Straight,
                (false, false) => return Rank::HighCard,
            },
            4 => return Rank::Pair,
            3 => {
                if counts.values().any(|&x| x == 2) {
                    return Rank::TwoPair;
                }
                return Rank::Set;
            }
            2 => {
                if counts.values().any(|&x| x == 2) {
                    return Rank::FullHouse;
                }
                return Rank::Bomb;
            }
            _ => panic!("no such rank invalid"),
        }
    }
}

impl TryFrom<&str> for HoldemHand {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = value
            .split_whitespace()
            .map(|s| Card::try_from(s))
            .collect::<Result<_, _>>()?;
        if cards.len() != 5 {
            return Err(Error::BadCard("invalid number of cards".to_string()));
        }
        Ok(Self::new(cards.try_into().unwrap()))
    }
}

impl Display for HoldemHand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tryfrom() {
        use crate::card::Suit::*;
        let hand = HoldemHand::try_from("2c 3c 4c 5c 6c").unwrap();
        assert_eq!(
            hand,
            HoldemHand([
                Card::new(Club, Value::Six),
                Card::new(Club, Value::Five),
                Card::new(Club, Value::Four),
                Card::new(Club, Value::Three),
                Card::new(Club, Value::Two)
            ])
        );
        println!("{hand}\n{hand:?}");
        //
        let hand = HoldemHand::try_from("2c 3c 4c 5c 6c 7c");
        assert_eq!(
            hand,
            Err(Error::BadCard("invalid number of cards".to_string()))
        );

        let hand = HoldemHand::try_from("2k 3c 4c 5c 6c");
        assert_eq!(hand, Err(Error::BadSuit("k".to_string())));
        let hand = HoldemHand::try_from("1s 3c 4c 5c 6c");
        assert_eq!(hand, Err(Error::BadValue("1".to_string())));
    }

    #[test]
    fn test_rank() {
        let hand = HoldemHand::try_from("As 10s Ks Qs js").unwrap();
        assert_eq!(hand.rank(), Rank::RoyalStraightFlush);

        let hand = HoldemHand::try_from("2c 3c 4c 5c Ac").unwrap();
        assert_eq!(hand.rank(), Rank::StraightFlush);

        let hand = HoldemHand::try_from("2s 9c 9s 9d 9h").unwrap();
        assert_eq!(hand.rank(), Rank::Bomb);

        let hand = HoldemHand::try_from("2c 2c 3c 3s 2h").unwrap();
        assert_eq!(hand.rank(), Rank::FullHouse);

        let hand = HoldemHand::try_from("2c 3c qc ac 9c").unwrap();
        assert_eq!(hand.rank(), Rank::Flush);

        let hand = HoldemHand::try_from("4c 3h 5d 7s 6s").unwrap();
        assert_eq!(hand.rank(), Rank::Straight);

        let hand = HoldemHand::try_from("2c 3h 2d 2s as").unwrap();
        assert_eq!(hand.rank(), Rank::Set);

        let hand = HoldemHand::try_from("2c 3h ad 3s 2s").unwrap();
        assert_eq!(hand.rank(), Rank::TwoPair);

        let hand = HoldemHand::try_from("2c kh ad js 2s").unwrap();
        assert_eq!(hand.rank(), Rank::Pair);

        let hand = HoldemHand::try_from("2c 3h ad ks 10s").unwrap();
        assert_eq!(hand.rank(), Rank::HighCard);
    }
}
