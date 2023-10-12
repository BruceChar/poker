use core::panic;
use std::{
    array,
    fmt::{Display, Formatter},
};

use crate::{
    card::{Card, Value},
    error::Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HoldemHand {
    cards: [Card; 5],
    rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    Pair([Value; 4]),
    TwoPair([Value; 3]),
    Set(Value),
    Straight,
    Flush,
    FullHouse(Value),
    Bomb(Value),
    StraightFlush,
    RoyalStraightFlush,
}

impl HoldemHand {
    fn new(mut cards: [Card; 5]) -> Self {
        cards.sort_by(|a, b| b.value().cmp(&a.value()));
        Self {
            cards,
            rank: Self::rank(&cards),
        }
    }

    fn rank(cards: &[Card; 5]) -> Rank {
        let mut counts = Vec::with_capacity(5);
        let mut is_flush = true;
        let mut is_straight = true;
        let mut pre = cards[0];
        counts.push((cards[0].value(), 1));
        let mut ind = 0;
        for cur in &cards[1..] {
            is_flush &= cur.suit() == pre.suit();
            is_straight &= cur.value() + 1 == pre.value()
                // "As 5c 4d 3h 2s" is straight
                || (pre.value() == Value::Ace && cur.value() == Value::Five);
            if cur.value() != pre.value() {
                counts.push((cur.value(), 1));
                ind += 1;
            } else {
                counts[ind].1 += 1;
            }
            pre = *cur;
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        match (counts.len(), is_flush, is_straight) {
            (5, true, true) => {
                if cards[1].value() == Value::King {
                    return Rank::RoyalStraightFlush;
                }
                return Rank::StraightFlush;
            }
            (5, true, false) => return Rank::Flush,
            (5, false, true) => return Rank::Straight,
            (5, false, false) => return Rank::HighCard,
            (4, _, _) => {
                return Rank::Pair(array::from_fn(|i| counts[i].0));
            }
            (3, _, _) => {
                if counts[0].1 == 2 {
                    return Rank::TwoPair(array::from_fn(|i| counts[i].0));
                }
                return Rank::Set(counts[0].0);
            }
            (2, _, _) => {
                if counts[0].1 == 3 {
                    return Rank::FullHouse(counts[0].0);
                }
                return Rank::Bomb(counts[0].0);
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
        Ok(Self::new(array::from_fn(|i| cards[i])))
    }
}

impl Display for HoldemHand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
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
            hand.cards,
            [
                Card::new(Club, Value::Six),
                Card::new(Club, Value::Five),
                Card::new(Club, Value::Four),
                Card::new(Club, Value::Three),
                Card::new(Club, Value::Two)
            ]
        );
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
        use Value::*;

        let hand = HoldemHand::try_from("As 10s Ks Qs js").unwrap();
        assert_eq!(hand.rank, Rank::RoyalStraightFlush);

        let hand = HoldemHand::try_from("2c 3c 4c 5c Ac").unwrap();
        assert_eq!(hand.rank, Rank::StraightFlush);

        let hand = HoldemHand::try_from("2s 9c 9s 9d 9h").unwrap();
        assert_eq!(hand.rank, Rank::Bomb(Value::Nine));

        let hand = HoldemHand::try_from("2c 2c 3c 3s 2h").unwrap();
        assert_eq!(hand.rank, Rank::FullHouse(Value::Two));

        let hand = HoldemHand::try_from("2c 3c qc ac 9c").unwrap();
        assert_eq!(hand.rank, Rank::Flush);

        let hand = HoldemHand::try_from("4c 3h 5d 7s 6s").unwrap();
        assert_eq!(hand.rank, Rank::Straight);

        let hand = HoldemHand::try_from("2c 3h 2d 2s as").unwrap();
        assert_eq!(hand.rank, Rank::Set(Value::Two));

        let hand = HoldemHand::try_from("2c 3h ad 3s 2s").unwrap();
        assert_eq!(hand.rank, Rank::TwoPair([Value::Three, Value::Two, Ace]));

        let hand = HoldemHand::try_from("2c kh ad js 2s").unwrap();
        assert_eq!(hand.rank, Rank::Pair([Two, Ace, King, Jack]));

        let hand = HoldemHand::try_from("2c 3h ad ks 10s").unwrap();
        assert_eq!(hand.rank, Rank::HighCard);
    }

    #[test]
    #[rustfmt::skip]
    fn test_rank_order() {
        use Rank::*;
        use Value::*;
        assert_eq!(HighCard, HighCard);
        assert_eq!(HighCard < TwoPair([Two, Ace, King]), true);
        assert_eq!(TwoPair([Two, Ace, King]) < TwoPair([Three, Ace, King]), true);
        assert_eq!(RoyalStraightFlush, RoyalStraightFlush);
        assert_eq!(RoyalStraightFlush < RoyalStraightFlush, false);
        assert_eq!(Bomb(Ace) > Bomb(King), true);
        assert_eq!(Bomb(King) > Bomb(Ace), false);
        assert_eq!(Bomb(King) > Bomb(King), false);
        assert_eq!(Bomb(King) > RoyalStraightFlush, false);
        assert_eq!(Bomb(King) > FullHouse(Ace), true);
        assert_eq!(Bomb(King) < StraightFlush, true);
        assert_eq!(Pair([Ace, King, Queen, Jack]) > Pair([Ace, Queen, Jack, Two]), true);
        assert_eq!(Pair([Ace, Queen, Jack, Three]) > Pair([Ace, Queen, Jack, Two]), true);
        assert_eq!(Pair([Ace, Queen, Jack, Three]), Pair([Ace, Queen, Jack, Three]));
    }
}
