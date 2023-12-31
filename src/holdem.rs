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
    HighCard([Value; 5]),
    Pair([Value; 4]),
    TwoPair([Value; 3]),
    Set([Value; 3]),
    Straight(Value),
    Flush([Value; 5]),
    FullHouse([Value; 2]),
    Bomb([Value; 2]),
    StraightFlush(Value),
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
        match counts.len() {
            5 => {
                let val = array::from_fn(|i| counts[i].0);
                if is_straight {
                    if is_flush && cards[1].value() == Value::King {
                        return Rank::RoyalStraightFlush;
                    }
                    let v = if cards[0].value() == Value::Ace {
                        cards[1].value()
                    } else {
                        cards[0].value()
                    };
                    if is_flush {
                        return Rank::StraightFlush(v);
                    }
                    return Rank::Straight(v);
                }
                if is_flush {
                    return Rank::Flush(val);
                }
                return Rank::HighCard(val);
            }
            4 => return Rank::Pair(array::from_fn(|i| counts[i].0)),
            3 => {
                let val = array::from_fn(|i| counts[i].0);
                if counts[0].1 == 2 {
                    return Rank::TwoPair(val);
                }
                return Rank::Set(val);
            }
            2 => {
                let val = array::from_fn(|i| counts[i].0);
                if counts[0].1 == 3 {
                    return Rank::FullHouse(val);
                }
                return Rank::Bomb(val);
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
        assert_eq!(hand.rank, Rank::StraightFlush(Five));

        let hand = HoldemHand::try_from("2s 9c 9s 9d 9h").unwrap();
        assert_eq!(hand.rank, Rank::Bomb([Value::Nine, Two]));

        let hand = HoldemHand::try_from("2c 2c 3c 3s 2h").unwrap();
        assert_eq!(hand.rank, Rank::FullHouse([Value::Two, Three]));

        let hand = HoldemHand::try_from("2c 3c qc ac 9c").unwrap();
        assert_eq!(hand.rank, Rank::Flush([Ace, Queen, Nine, Three, Two]));

        let hand = HoldemHand::try_from("4c 3h 5d 7s 6s").unwrap();
        assert_eq!(hand.rank, Rank::Straight(Seven));

        let hand = HoldemHand::try_from("2c 3h 2d 2s as").unwrap();
        assert_eq!(hand.rank, Rank::Set([Value::Two, Ace, Three]));

        let hand = HoldemHand::try_from("2c 3h ad 3s 2s").unwrap();
        assert_eq!(hand.rank, Rank::TwoPair([Value::Three, Value::Two, Ace]));

        let hand = HoldemHand::try_from("2c kh ad js 2s").unwrap();
        assert_eq!(hand.rank, Rank::Pair([Two, Ace, King, Jack]));

        let hand = HoldemHand::try_from("2c 3h ad ks 10s").unwrap();
        assert_eq!(hand.rank, Rank::HighCard([Ace, King, Ten, Three, Two]));

        //
        let first = HoldemHand::try_from("4c 3h 5d 2h 6s").unwrap();
        let second = HoldemHand::try_from("4c 3h 5d As 2s").unwrap();
        println!("{:?}", first.rank);
        println!("{:?}", second.rank);
        assert_eq!(first.rank > second.rank, true);
    }

    #[test]
    #[rustfmt::skip]
    fn test_rank_order() {
        use Rank::*;
        use Value::*;
        assert_eq!(HighCard([Ace, King, Ten, Three, Two]), HighCard([Ace, King, Ten, Three, Two]));
        assert_eq!(HighCard([Ace, King, Jack, Three, Two]) > HighCard([Ace, King, Ten, Three, Two]), true);
        assert_eq!(HighCard([Ace, King, Ten, Three, Two]) < TwoPair([Two, Ace, King]), true);
        assert_eq!(TwoPair([Two, Ace, King]) < TwoPair([Three, Ace, King]), true);
        assert_eq!(RoyalStraightFlush, RoyalStraightFlush);
        assert_eq!(RoyalStraightFlush < RoyalStraightFlush, false);
        assert_eq!(Bomb([Ace, Two]) > Bomb([King, Queen]), true);
        assert_eq!(Bomb([Ace, Three]) > Bomb([Ace, Two]), true);
        assert_eq!(Bomb([King, Queen]) > RoyalStraightFlush, false);
        assert_eq!(Bomb([King, Queen]) > FullHouse([Ace, Two]), true);
        assert_eq!(Bomb([King, Queen]) < StraightFlush(Ace), true);
        assert_eq!(Pair([Ace, King, Queen, Jack]) > Pair([Ace, Queen, Jack, Two]), true);
        assert_eq!(Pair([Ace, Queen, Jack, Three]) > Pair([Ace, Queen, Jack, Two]), true);
        assert_eq!(Pair([Ace, Queen, Jack, Three]), Pair([Ace, Queen, Jack, Three]));
        assert_eq!(Straight(Five) < Straight(Six), true);
    }
}
