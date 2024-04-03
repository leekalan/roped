use std::fmt::Display;

pub enum Error<Err> {
    Internal(InternalError),
    Err(Err),
}
impl<Err> Display for Error<Err>
where
    Err: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(v) => write!(f, "{}", v),
            Error::Err(v) => write!(f, "{}", v),
        }
    }
}

pub struct InternalError {
    pub index: usize,
    pub variant: ErrorType,
}
impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.variant {
            ErrorType::Expected(arg_type) => match arg_type {
                ArgType::Scope => write!(f, "Expected a scope ({})", self.index),
                ArgType::Arg => write!(f, "Expected an argument ({})", self.index),
                ArgType::Flag => write!(f, "Expected a flag ({})", self.index),
            },
            ErrorType::Parse(parse_err) => match parse_err.parse_type {
                ArgType::Scope => write!(
                    f,
                    "Scope \"{}\" does not exist ({})",
                    parse_err.arg, self.index
                ),
                ArgType::Arg => write!(
                    f,
                    "Unable to cast argument \"{}\" ({})",
                    parse_err.arg, self.index
                ),
                ArgType::Flag => unreachable!(),
            },
            ErrorType::InvalidFlag(flag) => {
                write!(f, "Flag \"--{}\" does not exist ({})", flag, self.index)
            }
            ErrorType::Unexpected(unexpected) => {
                write!(
                    f,
                    "Did not expect an argument \"{}\" ({})",
                    unexpected, self.index
                )
            }
        }
    }
}

pub enum ErrorType {
    Unexpected(String),
    Expected(ArgType),
    Parse(ParseErr),
    InvalidFlag(String),
}

pub struct ParseErr {
    pub arg: String,
    pub parse_type: ArgType,
}

pub enum ArgType {
    Scope,
    Arg,
    Flag,
}
