/// Error kind
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Dummy,
    Internal,
    Other,
    NotFound,
    PermissionDenied,
    Interrupted,
    Failure,
    LexerError,
    SyntaxError,
    EngineError,
    BrokenPipe,
    InvalidInput
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Dummy => "Dummy",
            ErrorKind::Internal => "Internal",
            ErrorKind::Other => "Other",
            ErrorKind::NotFound => "NotFound",
            ErrorKind::PermissionDenied => "PermissionDenied",
            ErrorKind::Interrupted => "Interrupted",
            ErrorKind::Failure => "Failure",
            ErrorKind::LexerError => "LexerError",
            ErrorKind::SyntaxError => "SyntaxError",
            ErrorKind::EngineError => "EngineError",
            ErrorKind::BrokenPipe => "BrokenPipe",
            ErrorKind::InvalidInput => "InvalidInput",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// Error
#[derive(Debug, PartialEq)]
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
