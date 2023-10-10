
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    BadHand,
    BadCard,
    BadSuit,
    BadRank,
    BadValue,
}