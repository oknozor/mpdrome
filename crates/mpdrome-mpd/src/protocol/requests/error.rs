#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Parse error {0}")]
    ParseError(String),
}
