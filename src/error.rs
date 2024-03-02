use std::fmt::Display;

pub enum RopedError<Err> where {
    Internal(RopedInternalError),
    Err(Err),
}

pub enum RopedInternalError {}
impl Display for RopedInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal Error")
    }
}