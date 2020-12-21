#[macro_export]
macro_rules! error {
    ($path:tt had $error:tt; $($arg:tt)*) => {
        ($path, $crate::error::Error::WithError(format!($($arg)*), Box::new($error)))
    };
    ($path:tt had $($arg:tt)*) => {
        ($path, $crate::error::Error::Simple(format!($($arg)*)))
    };
    ($error:tt; $($arg:tt)*) => {
        $crate::error::Error::WithError(format!($($arg)*), Box::new($error))
    };
    ($($arg:tt)*) => {
        $crate::error::Error::Simple(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($error:tt; $($arg:tt)*) => {
        return Err($crate::error::Error::WithError(format!($($arg)*), Box::new($error)));
    };
    ($($arg:tt)*) => {
        return Err($crate::error::Error::Simple(format!($($arg)*)));
    };
}

#[derive(Debug)]
pub enum Error {
    Simple(String),
    WithError(String, Box<dyn std::error::Error>),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(message) => write!(fmt, "{}", message),
            Self::WithError(message, error) => write!(fmt, "{}: {}", message, error),
        }
    }
}
