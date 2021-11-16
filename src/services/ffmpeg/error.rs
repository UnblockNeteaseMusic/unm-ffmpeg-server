//! FFmpeg 模組的錯誤部分。
use std::process::ExitStatus;
use std::string::FromUtf8Error;

use thiserror::Error;

/// FFmpeg 相關函式的錯誤。
#[derive(Error, Debug)]
pub enum FFmpegError {
    /// 無法啟動 FFmpeg 處理程序。
    ///
    /// * {0} - [`tokio::process::Command::output`] 所拋出的錯誤。
    /// * {1} - 完整命令。
    #[error("Failed to spawn {1}: {0}")]
    SpawnFFmpegFailed(std::io::Error, String),
    /// FFmpeg 執行失敗。
    ///
    /// * {0} - 處理程序回傳代碼。內容是 [`std::process::Output`] 的 `status`。
    #[error("FFmpeg exit unsuccessfully. (exit status {0})")]
    FFmpegRunFailed(ExitStatus),

    /// 無法強制停止 FFmpeg。
    ///
    /// * {0} - [`tokio::process::Child`] 之 `kill()` 所拋出的錯誤。
    #[error("Failed to kill ffmpeg: {0}")]
    FFmpegKillFailed(std::io::Error),
    /// 無法 `try_wait()`。
    ///
    /// * {0} - [`tokio::process::Child`] 之 `try_wait()` 所拋出的錯誤。
    #[error("Failed to try_wait(): {0}")]
    TryWaitFailed(std::io::Error),

    /// 無法將 UTF-8 vector 轉換成 String。
    ///
    /// * {0} - [`String::from_utf8`] 回傳的錯誤。
    #[error("Failed to convert a UTF-8 vector to String.")]
    ConvertUtf8Failed(#[from] FromUtf8Error),
}

/// FFmpeg 相關函式的回傳類型。
pub type FFmpegResult<T> = Result<T, FFmpegError>;
