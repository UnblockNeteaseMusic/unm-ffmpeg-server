//! UNM FFmpeg Server - 錯誤區塊
use thiserror::Error;

use crate::services::client::ClientServiceError;

/// 這個 FFmpeg 伺服器發生的錯誤。
#[derive(Error, Debug)]
pub enum ServerError {
    /// Client Service 發生的錯誤。
    #[error("services/client error: {0}")]
    ClientServiceError(ClientServiceError),
}

/// 這個 FFmpeg 伺服器的執行結果。
pub type ServerResult<T> = Result<T, ServerError>;
