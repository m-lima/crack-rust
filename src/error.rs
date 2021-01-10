#[macro_export]
macro_rules! error {
    ($error:tt; $($arg:tt)*) => {
        $crate::error::Error::new(format!("{}: {}", format!($($arg)*), $error))
    };
    ($($arg:tt)*) => {
        $crate::error::Error::new(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($error:tt; $($arg:tt)*) => {
        return Err($crate::error::Error::new(format!("{}: {}", format!($($arg)*), $error)));
    };
    ($($arg:tt)*) => {
        return Err($crate::error::Error::new(format!($($arg)*)));
    };
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

// Allowed because it matches the thread join
#[allow(clippy::needless_pass_by_value)]
pub fn on_join(err: Box<dyn std::any::Any + Send>) -> Error {
    if let Some(message) = err.downcast_ref::<&str>() {
        error!(message; "Thread error")
    } else if let Some(message) = err.downcast_ref::<String>() {
        error!(message; "Thread error")
    } else {
        error!("Thread error")
    }
}
