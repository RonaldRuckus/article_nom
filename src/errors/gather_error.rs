#[derive(Debug)]
pub enum GatherError {
    CmdError(fantoccini::error::CmdError),
    NewSessionError(fantoccini::error::NewSessionError),
    SessionDropped()
}

impl From<fantoccini::error::CmdError> for GatherError {
    fn from(err: fantoccini::error::CmdError) -> Self {
        GatherError::CmdError(err)
    }
}

impl From<fantoccini::error::NewSessionError> for GatherError {
    fn from(err: fantoccini::error::NewSessionError) -> Self {
        GatherError::NewSessionError(err)
    }
}
