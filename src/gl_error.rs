use std::fmt::Display;

pub enum GLGenericError {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    UnknownError,
}

impl Display for GLGenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            Self::NoError => "No Error",
            Self::InvalidEnum => "Invalid Enum",
            Self::InvalidValue => "Invalid Value",
            Self::InvalidOperation => "Invalid Operation",
            Self::StackOverflow => "Stack Overflow",
            Self::StackUnderflow => "Stack Underflow",
            Self::OutOfMemory => "Out of Memory",
            Self::UnknownError => "Unknown Error",
        };
        write!(f, "{}", err_msg)
    }
}

impl From<u32> for GLGenericError {
    fn from(value: u32) -> Self {
        match value {
            0x0000 => Self::NoError,
            0x0500 => Self::InvalidEnum,
            0x0501 => Self::InvalidValue,
            0x0502 => Self::InvalidOperation,
            0x0503 => Self::StackOverflow,
            0x0504 => Self::StackUnderflow,
            0x0505 => Self::OutOfMemory,
            _ => Self::UnknownError,
        }
    }
}

pub fn gl_clear_errors() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

pub fn gl_log_errors(file: impl Display, line: impl Display, statement: String) -> bool {
    let mut errored = false;
    let mut error: u32 = unsafe { gl::GetError() };
    while error != gl::NO_ERROR {
        let err_msg = format!("{}", GLGenericError::from(error));

        eprintln!("[OpenGL Error] ({}) {}", error, err_msg);
        eprintln!("\t│ Block starts @ {}:{}", file, line);
        eprintln!("\t└ Fault        @ {}\n", statement.replace('\n', ""));

        error = unsafe { gl::GetError() };
        errored = true;
        panic!("Stopped execution, OpenGL generated an error");
    }
    errored
}