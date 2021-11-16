//! FFmpeg 模組的作業相關 enum 部分。
use std::path::{Path, PathBuf};

use tokio::process::Child;

use super::format::Format;

/// FFmpeg 作業的狀態。
pub enum FFmpegStatus {
    /// 狀態未決。
    ///
    /// 這表示作業的狀態尚未確定。若狀態很久一段時間都卡在
    /// 「狀態未決」狀態，請查看日誌確定是否是程式遇到狀況。
    Determining,
    /// 等待執行。**目前沒用到。**
    ///
    /// 這個階段的作業尚未被 Executor 接手。
    Pending,
    /// 正在執行。
    ///
    /// 這個階段的作業已經被 Executor 接手，
    /// 且正在背景執行當中。
    Running,
    /// 已經完成。
    ///
    /// 這個階段的作業已經完成，可以取回。
    Completed,
    /// 被中止。
    ///
    /// 這個階段的作業雖然已經被 Executor 接手過，
    /// 但執行時發生錯誤，作業沒有完成。
    ///
    /// * `{1}` 是程序結束碼。假如沒有，則為 None。
    Interrupted(Option<i32>),
}

/// 一個 FFmpeg 作業。
pub struct FFmpegTask {
    /// FFmpeg 作業的狀態。
    pub status: FFmpegStatus,
    /// FFmpeg 作業的處理程序結構體。
    pub child: Child,
    /// FFmpeg 作業產出檔案的目的地。
    pub target: PathBuf,
}

/// FFmpeg 作業的參數。
pub struct FFmpegTaskParameters<'a> {
    /// FFmpeg 的目標格式。
    pub format: Format,
    /// 來源檔案。
    pub src: &'a Path,
    /// 目標檔案。
    pub target: &'a Path,
}
