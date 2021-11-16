//! FFmpeg 模組的作業執行器 (Executor) 部分。
//!
//! # 範例
//!
//! todo
use std::ffi::OsStr;

use log::info;
use tokio::process::Command;

use super::error::{FFmpegError, FFmpegResult};
use super::task::{FFmpegStatus, FFmpegTask, FFmpegTaskParameters};

/// 一個標準執行器 (Executor)。
pub trait Executor {
    /// 作業參數。
    ///
    /// 要傳入 executor 的參數。
    type TaskParameter;
    /// 可供取用者追蹤的作業。
    ///
    /// Task 通常包括作業情境 (context) 以及
    /// 目前作業的狀態。
    type Task;

    /// 處理指定的作業。
    fn executor(&self, param: &Self::TaskParameter) -> Self::Task;
}

/// FFmpeg 作業執行器（Executor）。
pub struct FFmpegExecutor<'a> {
    /// FFmpeg 可執行檔的名稱或位置。
    pub executable: &'a OsStr,
}

impl<'a> Executor for FFmpegExecutor<'a> {
    type TaskParameter = FFmpegTaskParameters<'a>;
    type Task = FFmpegResult<FFmpegTask>;

    fn executor(&self, param: &Self::TaskParameter) -> Self::Task {
        let mut command = Command::new(self.executable);

        // Prepare the arguments:
        //     ffmpeg -y -i {param.src} [...] {param.target}
        {
            // ffmpeg -y -i {param.src} ...
            command.arg("-y").arg("-i").arg(param.src);

            // Set the codec.
            command.arg("-c:a").arg(param.format.get_codec());

            // Set the bitrate.
            if let Some(bitrate) = param.format.get_bitrate() {
                command.arg("-b:a").arg(bitrate.to_string());
            }

            // ... {param.target}
            command.arg(param.target);
        }

        // Format the command, and print a log for it.
        let command_str = format!("{:?}", command);
        info!("ffmpeg > executor: executing: {}", command_str);

        // Execute the command.
        let child = command
            .spawn()
            .map_err(|err| FFmpegError::SpawnFFmpegFailed(err, command_str))?;

        Ok(FFmpegTask {
            status: FFmpegStatus::Running,
            child,
            target: param.target.to_path_buf(),
        })
    }
}
