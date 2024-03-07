/// Error kind
#[derive(Debug)]
pub enum ErrorKind {
    Dummy,
    Internal,
    Other,
    NotFound,
    PermissionDenied,

    SyntaxError,
}

/// Error
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    // message: &'static str,
    message: String,
}

impl Error {
    /// Dummy error
    pub const DUMMY: Self = Self {
        kind: ErrorKind::Dummy,
        message: String::new(),
    };

    /// Create new error
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        let message = message.to_string();
        Self { kind, message }
    }

    // pub fn new_with_string(kind: ErrorKind, message: String) -> Self {
    //     Self {
    //         kind,
    //         message: Box::leak(message.into_boxed_str()),
    //     }
    // }

    /// Get error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Get error message
    pub fn message(&self) -> &str {
        &self.message
    }
}
