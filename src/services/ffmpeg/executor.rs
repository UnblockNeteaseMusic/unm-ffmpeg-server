//! FFmpeg 模組的作業執行器 (Executor) 部分。
//!
//! # 範例
//!
//! ```
//! let executor = FFmpegExecutor {
//!   executable: "echo".as_ref(),
//!  };
//!
//!  let root = Path::new("/tmp/ffmpeg-storage");
//!  let result = executor
//!      .executor(&FFmpegTaskParameters {
//!          format: Format::Flac,
//!          src: &*root.join("src.mp4"),
//!          target: &*root.join("target.flac"),
//!      })
//!       .unwrap();
//! ```
use std::ffi::OsStr;
use std::process::Stdio;

use log::info;
use tokio::process::Command;

use super::error::{FFmpegServiceError, FFmpegServiceResult};
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
    type Task = FFmpegServiceResult<FFmpegTask>;

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
                let mut br = bitrate.to_string();
                br.push('k');
                command.arg("-b:a").arg(br);
            }

            // ... {param.target}
            command.arg(param.target);
        }

        // Format the command, and print a log for it.
        let command_str = format!("{:?}", command);
        info!("ffmpeg > executor: executing: {}", command_str);

        // Execute the command.
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| FFmpegServiceError::SpawnFFmpegFailed(err, command_str))?;

        Ok(FFmpegTask {
            status: FFmpegStatus::Running,
            child,
            target: param.target.to_path_buf(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::services::ffmpeg::executor::{Executor, FFmpegExecutor};
    use crate::services::ffmpeg::format::Format;
    use crate::services::ffmpeg::task::FFmpegTaskParameters;
    use std::path::Path;

    #[tokio::test]
    async fn executor_flac_test() {
        let executor = FFmpegExecutor {
            executable: "echo".as_ref(),
        };

        let root = Path::new("/");
        let result = executor
            .executor(&FFmpegTaskParameters {
                format: Format::Flac,
                src: &*root.join("src.mp4"),
                target: &*root.join("target.flac"),
            })
            .unwrap();

        let child = result.child;
        let output = child.wait_with_output().await.unwrap();
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        assert_eq!(output_str.trim(), "-y -i /src.mp4 -c:a flac /target.flac");
    }

    #[tokio::test]
    async fn executor_mp3_test() {
        let executor = FFmpegExecutor {
            executable: "echo".as_ref(),
        };

        let root = Path::new("/");
        let result = executor
            .executor(&FFmpegTaskParameters {
                format: Format::Mp3(320),
                src: &*root.join("src.mp4"),
                target: &*root.join("target.mp3"),
            })
            .unwrap();

        let child = result.child;
        let output = child.wait_with_output().await.unwrap();
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        assert_eq!(
            output_str.trim(),
            "-y -i /src.mp4 -c:a libmp3lame -b:a 320k /target.mp3"
        );
    }

    #[tokio::test]
    async fn executor_aac_test() {
        let executor = FFmpegExecutor {
            executable: "echo".as_ref(),
        };

        let root = Path::new("/");
        let result = executor
            .executor(&FFmpegTaskParameters {
                format: Format::Aac(128),
                src: &*root.join("src.mp4"),
                target: &*root.join("target.aac"),
            })
            .unwrap();

        let child = result.child;
        let output = child.wait_with_output().await.unwrap();
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        assert_eq!(
            output_str.trim(),
            "-y -i /src.mp4 -c:a aac -b:a 128k /target.aac"
        );
    }
}
