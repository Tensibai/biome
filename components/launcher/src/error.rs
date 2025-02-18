use crate::protocol;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[warn(clippy::result_large_err)]
pub enum ServiceRunError {
    #[error("Failed to spawn service process")]
    Spawn(#[source] io::Error),
    #[cfg(unix)]
    #[error("Failed to determine UID for user '{0}'")]
    GetUid(String, #[source] biome_core::Error), // JAH: GetUid largest variant >= 128 bytes
    #[cfg(unix)]
    #[error("Failed to determine GID for group '{0}'")]
    GetGid(String, #[source] biome_core::Error), // JAH: GetGid largest variant >= 128 bytes
    #[cfg(windows)]
    #[error("Failed to determine current username")]
    GetCurrentUsername(#[source] biome_core::Error),
    #[error("No GID for group '{0}' could be found")]
    GroupNotFound(String),
    #[error("No UID for user '{0}' could be found")]
    UserNotFound(String),
}

impl From<ServiceRunError> for protocol::ErrCode {
    fn from(err: ServiceRunError) -> protocol::ErrCode {
        match err {
            ServiceRunError::GroupNotFound(_) => protocol::ErrCode::GroupNotFound,
            ServiceRunError::UserNotFound(_) => protocol::ErrCode::UserNotFound,
            _ => protocol::ErrCode::Unknown,
        }
    }
}
