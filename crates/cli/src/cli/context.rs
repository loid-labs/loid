use console::{Term, style};
use std::path::PathBuf;

/// Represents the cli context for the application.
pub struct Context {
    /// The verbosity level
    pub verbose: u8,
    pub api_url: String,
    pub cwd: PathBuf,
    pub stdout: Term,
    pub stderr: Term,
}

impl Context {
    pub fn load(verbose: u8, api_url: String, cwd: PathBuf) -> Self {
        Self {
            verbose,
            api_url,
            cwd,
            stdout: Term::stdout(),
            stderr: Term::stderr(),
        }
    }

    /// Write a message to stdout
    pub fn print(&self, message: &str) -> std::io::Result<()> {
        self.stdout.write_line(message)
    }

    /// Write an error message to stderr
    pub fn error(&self, message: &str) -> std::io::Result<()> {
        self.stderr.write_line(&format!("{}", style(message).red()))
    }

    /// Write a success message to stdout
    pub fn success(&self, message: &str) -> std::io::Result<()> {
        self.stdout
            .write_line(&format!("{}", style(message).green()))
    }

    /// Write an info message to stdout (only if verbose)
    pub fn info(&self, message: &str) -> std::io::Result<()> {
        if self.verbose > 0 {
            self.stdout.write_line(&format!("{}", style(message).dim()))
        } else {
            Ok(())
        }
    }
}
