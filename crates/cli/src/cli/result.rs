use anyhow::anyhow;
use std::process::ExitCode;

#[derive(Debug)]
pub struct CliError {
    /// The error to display. This can be `None` in rare cases to exit with a
    /// code without displaying a message.
    pub error: Option<anyhow::Error>,
    /// The process exit code.
    pub exit_code: u8,
}

impl CliError {
    pub fn new(error: anyhow::Error, code: u8) -> Self {
        Self {
            error: Some(error),
            exit_code: code,
        }
    }

    pub fn code(code: u8) -> Self {
        Self {
            error: None,
            exit_code: code,
        }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        Self::new(err, 101)
    }
}

impl From<clap::Error> for CliError {
    fn from(err: clap::Error) -> Self {
        let code = if err.use_stderr() { 1 } else { 0 };
        Self::new(err.into(), code)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.into(), 1)
    }
}

impl From<tonic::transport::Error> for CliError {
    fn from(value: tonic::transport::Error) -> Self {
        Self::new(anyhow!("Failed to connect to server: {}", value), 1)
    }
}

impl From<tonic::Status> for CliError {
    fn from(value: tonic::Status) -> Self {
        Self::new(value.into(), 1)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ExitStatus {
    /// The command succeeded.
    Success,

    /// The command failed due to an error in the user input.
    Failure,

    /// The command failed with an unexpected error.
    Error,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => Self::from(0),
            ExitStatus::Failure => Self::from(1),
            ExitStatus::Error => Self::from(2),
        }
    }
}

/// Defines a convenience type for a `Result` that returns a `CliError` on failure.
pub type CliResult = Result<ExitStatus, CliError>;

#[cfg(test)]
mod tests {
    use std::io;

    use anyhow::anyhow;

    use super::*;

    #[test]
    fn test_new() {
        let cli_error = CliError::new(anyhow!("test error"), 100);
        assert_eq!(cli_error.exit_code, 100);
        assert!(cli_error.error.is_some());
    }

    #[test]
    fn test_code() {
        let cli_error = CliError::code(101);
        assert_eq!(cli_error.exit_code, 101);
        assert!(cli_error.error.is_none());
    }

    #[test]
    fn test_from_anyhow() {
        let err = anyhow!("anyhow error");
        let cli_error: CliError = err.into();
        assert_eq!(cli_error.exit_code, 101);
        assert!(cli_error.error.is_some());
    }

    #[test]
    fn test_from_clap() {
        let err = clap::Error::raw(clap::error::ErrorKind::InvalidValue, "clap error");
        let cli_error: CliError = err.into();
        assert_eq!(cli_error.exit_code, 1);
        assert!(cli_error.error.is_some());
    }

    #[test]
    fn test_from_io() {
        let err = io::Error::new(io::ErrorKind::NotFound, "io error");
        let cli_error: CliError = err.into();
        assert_eq!(cli_error.exit_code, 1);
        assert!(cli_error.error.is_some());
    }
}
