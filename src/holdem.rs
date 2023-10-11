use core::panic;
use std::fmt::{Display, Formatter};

use crate::{
    card::{Card, Value},
    error::Error,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct HoldemHand([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard = 1,
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

    ///
    fn rank(&self) -> Rank {
        let mut cards = self.0;
        let mut pairs = vec![Vec::with_capacity(4); 2];
        let mut high = Vec::with_capacity(5);

        let mut is_flush = true;
        let mut is_straight = true;

        let mut pre = cards[0];
        let mut pre2 = cards[0];
        for (i, cur) in cards[1..].iter().enumerate() {
            let is_last = i == 4;
            if is_flush && cur.suit() != pre.suit() {
                is_flush = false;
            }
            if is_straight && cur.value() + 1 != pre.value(){
                // "As 5c 4d 3h 2s"
                if !(pre.value() == Value::Ace
                    && cur.value() == Value::Five)

                {
                    is_straight = false;
                }
            }
            // pair, set, bomb
            let p0e = pairs[0].is_empty();
            let p1e = pairs[1].is_empty();
            if cur.value() == pre.value() {
                let buf = if i == 3 { vec![pre, *cur] } else { vec![pre] };
                if p0e {
                    pairs[0].extend(buf);
                } else {
                    if pre.value() == pre2.value() {
                        pairs[0].extend(buf);
                    } else {
                        pairs[1].extend(buf)
                    }
                }
            } else {
                // two pair?
                if p0e || pre.value() != pre2.value() {
                    high.push(pre);
                } else {
                    if pre.value() == pre2.value() {
                        if !p1e {
                            pairs[1].push(pre);
                        } else {
                            pairs[0].push(pre);
                        }
                    }
                }
                if i == 3 {
                    high.push(*cur);
                }
            }
            pre2 = pre;
            pre = *cur;
        }
        if is_flush && is_straight {
            // royal_flush_straight
            if cards[1].value() == Value::King {
                return Rank::RoyalStraightFlush;
            }
            return Rank::StraightFlush;
        }
        if is_flush {
            return Rank::Flush;
        }
        if is_straight {
            return Rank::Straight;
        }
        match high.len() {
            5 => return Rank::HighCard,
            3 => return Rank::Pair,
            2 => return Rank::Set,
            1 => {
                if pairs[1].len() == 0 {
                    return Rank::Bomb;
                }
                return Rank::TwoPair;
            }
            0 => return Rank::FullHouse,
            _ => panic!("no such rank invalid"),
        }

        // if !pairs[0].is_empty() && !pairs[1].is_empty() {
        //     // fullhouse or two pair

        // }
    }

    fn is_high_card(&self) -> bool {
        self.0
            .iter()
            .map(|card| card.value())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] != w[1])
    }

    fn is_pair(&self) -> bool {
        self.0
            .iter()
            .map(|card| card.value())
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w[0] == w[1])
    }

    fn is_two_pair(&self) -> bool {
        let pairs = self
            .0
            .iter()
            .map(|card| card.value())
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|w| w[0] == w[1])
            .count();
        pairs == 2
    }

    fn is_set(&self) -> bool {
        let sets = self
            .0
            .iter()
            .map(|card| card.value())
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|w| w[0] == w[1] && w[1] == w[2])
            .count();
        sets == 1
    }

    fn is_straight(&self) -> bool {
        self.0
            .windows(2)
            .all(|w| w[0].value() as i32 - w[1].value() as i32 == 1)
    }

    fn is_full_house(&self) -> bool {
        self.is_set() && self.is_pair()
    }

    fn is_bomb(&self) -> bool {
        let bombs = self
            .0
            .iter()
            .map(|card| card.value())
            .collect::<Vec<_>>()
            .windows(4)
            .filter(|w| w[0] == w[1] && w[1] == w[2] && w[2] == w[3])
            .count();
        bombs == 1
    }

    fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    fn is_flush(&self) -> bool {
        let suit = self.0[0].suit();
        self.0.iter().all(|card| card.suit() == suit)
    }

    fn is_royal_straight_flush(&self) -> bool {
        self.0[0].value() == Value::Ace && self.is_straight() && self.is_flush()
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
