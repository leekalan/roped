use std::fmt::Display;

pub enum Error<T, Err> {
    Internal(InternalError<T>),
    Err(Err),
}
impl<T, Err> Display for Error<T, Err>
where
    Err: Display,
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(v) => write!(f, "{}", v),
            Error::Err(v) => write!(f, "{}", v),
        }
    }
}

pub struct InternalError<T> {
    index: usize,
    variant: ErrorType<T>,
}
impl<T> Display for InternalError<T>
where
    T: Display,
{
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
            ErrorType::Flag(flag) => write!(f, "Invalid flag \"--{}\"", flag),
        }
    }
}

pub enum ErrorType<T> {
    Expected(ArgType),
    Parse(ParseErr<T>),
    Flag(T),
}

pub struct ParseErr<T> {
    arg: T,
    parse_type: ArgType,
}

pub enum ArgType {
    Scope,
    Arg,
}
