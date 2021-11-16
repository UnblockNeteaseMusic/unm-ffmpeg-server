use thiserror::Error;

use crate::services::client::ClientServiceError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("services/client error: {0}")]
    ClientServiceError(ClientServiceError),
}

pub type ServerResult<T> = Result<T, ServerError>;
