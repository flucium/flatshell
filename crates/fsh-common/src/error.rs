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
    InvalidInput,
}

impl ErrorKind {
    pub const fn as_str(&self) -> &'static str {
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

    pub const fn as_str2(&self) -> &'static str {
        match self {
            ErrorKind::Dummy => "dummy",
            ErrorKind::Internal => "internal",
            ErrorKind::Other => "other",
            ErrorKind::NotFound => "not found",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::Interrupted => "interrupted",
            ErrorKind::Failure => "failure",
            ErrorKind::LexerError => "lexer error",
            ErrorKind::SyntaxError => "syntax error",
            ErrorKind::EngineError => "engine error",
            ErrorKind::BrokenPipe => "broken pipe",
            ErrorKind::InvalidInput => "invalid input",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    // message: &'static str,
    message: String,
}

impl Error {
    pub const DUMMY: Self = Self {
        kind: ErrorKind::Dummy,
        message: String::new(),
    };

    pub const INTERNAL: Self = Self {
        kind: ErrorKind::Internal,
        message: String::new(),
    };

    pub fn new(kind: ErrorKind, message: &str) -> Self {
        let message = message.to_string();
        Self { kind, message }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
