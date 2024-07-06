use core::fmt;

#[derive(Debug, Clone)]
enum AnalyserErrorKind {
    Type,
    Name,
    Argument,
    IllegalDeclaration,
}

impl fmt::Display for AnalyserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "[Type Error]"),
            Self::Name => write!(f, "[Name Error]"),
            Self::Argument => write!(f, "[Argument Error]"),
            Self::IllegalDeclaration => write!(f, "[Illegal Declaration]"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalyserError {
    kind: AnalyserErrorKind,
    msg: String,
}

impl AnalyserError {
    pub fn type_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Type,
            msg,
        }
    }

    pub fn name_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Name,
            msg,
        }
    }

    pub fn arg_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Argument,
            msg,
        }
    }

    pub fn illegal_decl(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::IllegalDeclaration,
            msg,
        }
    }
}

impl fmt::Display for AnalyserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type AnalyserErrors = Vec<AnalyserError>;
