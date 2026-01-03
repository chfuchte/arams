pub(crate) enum StatusCode {
    NoError = 0,
    UserError = 1,
    SystemError = 2,
}

impl From<StatusCode> for i32 {
    fn from(status_code: StatusCode) -> Self {
        status_code as i32
    }
}

#[derive(Debug)]
pub(crate) enum Error {
    FailedToParseArgs(clap::Error),
    IOFailedToReadFile(std::io::Error),
    IOFailedToReadFromStdIn(std::io::Error),
    NoInput,
}

impl Error {
    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            Error::FailedToParseArgs(_) => StatusCode::SystemError,
            Error::IOFailedToReadFile(_) => StatusCode::SystemError,
            Error::IOFailedToReadFromStdIn(_) => StatusCode::SystemError,
            Error::NoInput => StatusCode::UserError,
        }
    }

    pub(crate) fn standalone_message(&self) -> String {
        match self {
            Error::NoInput => {
                "No input provided. Please add a file path, a raw string argument, or pipe data to stdin."
                    .to_string()
            }
            _ => format!("{}", self),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedToParseArgs(e) => write!(
                f,
                "Failed to parse arguments\nHint: check the usage with --help\n{}",
                e
            ),
            Error::IOFailedToReadFile(e) => write!(
                f,
                "Failed to read file\nTip: verify the file exists and permissions are correct.\n{}",
                e
            ),
            Error::IOFailedToReadFromStdIn(e) => write!(f, "Failed to read from stdin\n{}", e),
            Error::NoInput => {
                write!(
                    f,
                    "No input provided. Please provide a file path, a raw string argument, or pipe data into stdin."
                )
            }
        }
    }
}

impl std::error::Error for Error {}
