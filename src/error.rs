use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("Bad value: {0}")]
    BadValue(String),

    #[error("Bad suit: {0}")]
    BadSuit(String),

    #[error("Bad card: {0}")]
    BadCard(String),

    #[error("Bad rank error")]
    BadRank,

    #[error("Bad hand error")]
    BadHand,
}
