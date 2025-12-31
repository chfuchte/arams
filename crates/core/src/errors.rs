#[derive(Debug)]
pub enum ARAMSError {}

impl std::fmt::Display for ARAMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ARAMSError {}
