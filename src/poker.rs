use crate::card::*;
struct Pack {
    values: Vec<Value>,
    suits: Vec<Suit>,
    jokers: Option<Vec<Joker>>,
}

impl Pack {
    fn default() -> Self {
        Pack {
            values: Value::values().into(),
            suits: Suit::values().into(),
            jokers: Some(vec![Joker::Big, Joker::Small]),
        }
    }
}

trait Poker {
    
}

trait Rank {
    fn rank(&self) -> u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        let pack = Pack::default();
        assert_eq!(pack.values.len(), 13);
        assert_eq!(pack.suits.len(), 4);
        assert_eq!(pack.jokers.unwrap().len(), 2);
    }
}