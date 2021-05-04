use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    ArgumentTypes,
    StackEmpty,
    BadStackIndex(usize, usize),
    UndefinedGlobal(String),
    UndefinedProperty(String),
    ReturnFromTopLevel,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArgumentTypes => write!(f, "Incompatible types for operation"),
            Self::StackEmpty => write!(f, "Tried to pop value from empty stack"),
            Self::BadStackIndex(wanted, len) => write!(
                f,
                "Tried to access value at index {} beyond end of stack (height {})",
                wanted, len
            ),
            Self::UndefinedGlobal(name) => {
                write!(f, "Tried to access undefined variable `{}`", name)
            }
            Self::UndefinedProperty(name) => write!(
                f,
                "Tried to access undefined property `{}` on instance",
                name
            ),
            Self::ReturnFromTopLevel => write!(
                f,
                "Cannot return from top-level.",
            ),
        }
    }
}