#![allow(unused_imports)]
use std::{fmt::{Display, Formatter}, ops::{Range, RangeBounds, Bound}};

use crate::error::Error;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}
impl Suit {
    pub fn values() -> [Self; 4] {
        [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade]
    }
}

impl TryFrom<&str> for Suit {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "h" | "H" => Ok(Suit::Heart),
            "d" | "D" => Ok(Suit::Diamond),
            "c" | "C" => Ok(Suit::Club),
            "s" | "S" => Ok(Suit::Spade),
            _ => Err(Error::BadSuit),
        }
    }
}

#[rustfmt::skip]
impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Suit::Heart => "h",
            Suit::Diamond => "d",
            Suit::Club => "c",
            Suit::Spade => "s",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Value {
    Two = 2,
    Three,
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn values() -> [Value; 13] {
        use Value::*;
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
    }
}

impl std::ops::Add<u8> for Value {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
       self.value().add(rhs)
    }
}

impl std::ops::Add<Value> for u8 {
    type Output = u8;
    fn add(self, rhs: Value) -> Self::Output {
        self.add(rhs.value())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::values()[value as usize]
    }
}

impl PartialEq<Value> for u8 {
    fn eq(&self, other: &Value) -> bool {
        *self == other.value()
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, other: &u8) -> bool {
        self.value() == *other
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" | "A" => Ok(Value::Ace),
            "k" | "K" => Ok(Value::King),
            "q" | "Q" => Ok(Value::Queen),
            "j" | "J" => Ok(Value::Jack),
            "10" => Ok(Value::Ten),
            "9" => Ok(Value::Nine),
            "8" => Ok(Value::Eight),
            "7" => Ok(Value::Seven),
            "6" => Ok(Value::Six),
            "5" => Ok(Value::Five),
            "4" => Ok(Value::Four),
            "3" => Ok(Value::Three),
            "2" => Ok(Value::Two),
            _ => Err(Error::BadValue),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ace => write!(f, "A"),
            Value::King => write!(f, "K"),
            Value::Queen => write!(f, "Q"),
            Value::Jack => write!(f, "J"),
            other => write!(f, "{}", other.value()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Joker {
    Small,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(Suit, Value);

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self(suit, value)
    }

    pub fn suit(&self) -> Suit {
        self.0
    }

    pub fn value(&self) -> Value {
        self.1
    }
}

impl TryFrom<&str> for Card {
    type Error = Error;

    fn try_from(card: &str) -> Result<Self, Self::Error> {
        let len = card.len();
        if len != 2 && len != 3 {
            return Err(Error::BadCard);
        }
        match card.split_at(len - 1) {
            (v, s) => Ok(Self(Suit::try_from(s)?, Value::try_from(v)?)),
            _ => Err(Error::BadCard)
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit() {
        assert_eq!(Suit::try_from("h"), Ok(Suit::Heart));
        assert_eq!(Suit::try_from("d"), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from("c"), Ok(Suit::Club));
        assert_eq!(Suit::try_from("s"), Ok(Suit::Spade));
        assert_eq!(Suit::try_from("H"), Ok(Suit::Heart));
        assert_eq!(Suit::try_from("D"), Ok(Suit::Diamond));
        assert_eq!(Suit::try_from("C"), Ok(Suit::Club));
        assert_eq!(Suit::try_from("S"), Ok(Suit::Spade));
        assert_eq!(Suit::try_from("x"), Err(Error::BadSuit));
        assert_eq!(Suit::try_from(""), Err(Error::BadSuit));
    }

    #[test]
    fn test_value() {
        assert_eq!(Value::try_from("a"), Ok(Value::Ace));
        assert_eq!(Value::try_from("A"), Ok(Value::Ace));
        assert_eq!(Value::try_from("2"), Ok(Value::Two));
        assert_eq!(Value::try_from("10"), Ok(Value::Ten));
        assert_eq!(Value::try_from("13"), Err(Error::BadValue));
        assert_eq!(Value::try_from("0"), Err(Error::BadValue));
        assert_eq!(Value::try_from("1"), Err(Error::BadValue));

        // eq
        assert_ne!(Value::Ace, Value::Two);
        assert_eq!(Value::Ace, Value::Ace);
        assert_eq!(Value::Ace, 14);
        assert_ne!(Value::Ace, 1);
        assert_ne!(1, Value::Ace);
        assert_eq!(14, Value::Ace);

        // ord
        assert_eq!(Value::Two < Value::Ace, true);

        // Add
        assert_eq!(Value::Ace + 1, 15);
        assert_eq!(1 + Value::Ten, 11);
    }

    #[test]
    #[rustfmt::skip]
    fn test_card() {
        assert_eq!(Card::try_from("2h"), Ok(Card(Suit::Heart, Value::Two)));
        assert_eq!(Card::try_from("2H"), Ok(Card(Suit::Heart, Value::Two)));
        assert_eq!(Card::try_from("Ad"), Ok(Card(Suit::Diamond, Value::Ace)));
        assert_eq!(Card::try_from("aD"), Ok(Card(Suit::Diamond, Value::Ace)));
        assert_eq!(Card::try_from("10d"), Ok(Card(Suit::Diamond, Value::Ten)));

        // bad suit to parse
        assert_eq!(Card::try_from("Ak"), Err(Error::BadSuit));
        assert_eq!(Card::try_from("pk"), Err(Error::BadSuit)); // parse suit first

        // bad value to parse
        assert_eq!(Card::try_from("pD"), Err(Error::BadValue));
        assert_eq!(Card::try_from("20D"), Err(Error::BadValue));
        assert_eq!(Card::try_from("0D"), Err(Error::BadValue));
        assert_eq!(Card::try_from("*D"), Err(Error::BadValue));

        // bad card format
        assert_eq!(Card::try_from("100D"), Err(Error::BadCard));
        assert_eq!(Card::try_from("*"), Err(Error::BadCard));
        assert_eq!(Card::try_from(""), Err(Error::BadCard));
    }
}
