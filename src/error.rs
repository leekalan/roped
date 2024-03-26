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
    index: usize,
    variant: ErrorType,
}
impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.variant {
            ErrorType::Expected(arg_type) => match arg_type {
                ArgType::Scope => write!(f, "Missing scope ({})", self.index),
                ArgType::Arg => write!(f, "Missing argument ({})", self.index),
            },
            ErrorType::Parse(parse_err) => match parse_err.parse_type {
                ArgType::Scope => write!(f, "Invalid scope \"{}\" ({})", parse_err.arg, self.index),
                ArgType::Arg => write!(
                    f,
                    "Unable to cast argument \"{}\" ({})",
                    parse_err.arg, self.index
                ),
            },
            ErrorType::Flag(flag) => write!(f, "Invalid flag \"--{}\" ({})", flag, self.index),
        }
    }
}

pub enum ErrorType {
    Expected(ArgType),
    Parse(ParseErr),
    Flag(String),
}

pub struct ParseErr {
    arg: String,
    parse_type: ArgType,
}

pub enum ArgType {
    Scope,
    Arg,
}
