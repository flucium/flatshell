/// Error kind
#[derive(Debug)]
pub enum ErrorKind {
    Dummy,
    Internal,
    Other,
    NotFound,
    PermissionDenied,

    Syntax,
}

/// Error
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Error {
    /// Dummy error
    pub const DUMMY: Self = Self {
        kind: ErrorKind::Dummy,
        message: "Dummy error",
    };

    /// Create new error
    pub fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    /// Get error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Get error message
    pub fn message(&self) -> &'static str {
        self.message
    }
}
