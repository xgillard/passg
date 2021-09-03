
#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Error {
    #[error("could not parse")]
    ParseError(String)
}
