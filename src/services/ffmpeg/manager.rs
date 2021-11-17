//! FFmpeg 模組的處理程序管理結構體部分。
//!
//! # 範例
//!
//! todo
use std::collections::HashMap;

use log::trace;

use crate::services::ffmpeg::error::FFmpegServiceError::{FFmpegKillFailed, TryWaitFailed};
use crate::services::ffmpeg::executor::{Executor, FFmpegExecutor};
use crate::services::ffmpeg::task::FFmpegStatus::Interrupted;
use crate::services::ffmpeg::task::{FFmpegStatus, FFmpegTaskParameters};

use super::error::FFmpegServiceResult;
use super::task::FFmpegTask;

/// FFmpeg 處理程序管理結構體。
pub struct FFmpegManager<'a> {
    /// FFmpeg 執行器。
    pub executor: FFmpegExecutor<'a>,
    /// 登記在 FFmpegManager 的作業名單。
    tasks: HashMap<String, FFmpegTask>,
}

impl<'a> FFmpegManager<'a> {
    /// 建立一個新的 FFmpeg 處理程序管理結構體。
    pub fn new(executor: FFmpegExecutor<'a>) -> Self {
        FFmpegManager {
            executor,
            tasks: HashMap::new(),
        }
    }

    /// 加入作業。
    ///
    /// `ident` 是本次作業的唯一識別代號；
    /// `param` 是要傳入 Executor 的參數。
    ///
    /// 若已經註冊過同名稱的 `ident`，則會取消並覆蓋原先存在的作業。
    pub fn add_task(&mut self, ident: String, param: TaskParameter) -> FFmpegServiceResult<()> {
        trace!("adding {ident} to FFmpegManager...", ident = ident);
        self.tasks.insert(ident, self.executor.executor(&param)?);
        Ok(())
    }

    /// 取得作業的最新狀態。
    ///
    /// `ident` 是欲尋找作業的唯一識別代號。
    ///
    /// 若作業存在，則回傳 `Some(FFmpegTask {...})`；
    /// 反之，回傳 `None`。
    pub fn retrieve_task(&mut self, ident: &str) -> FFmpegServiceResult<Option<&mut FFmpegTask>> {
        trace!("retrieving {ident} from FFmpegManager...", ident = ident);
        let mut task = self.tasks.get_mut(ident);

        if let Some(ref mut task) = task {
            trace!("updating the status of {ident}...", ident = ident);
            update_status(task)?;
        }

        Ok(task)
    }

    /// 中止作業。
    ///
    /// `ident` 是欲尋找作業的唯一識別代號。
    ///
    /// 若作業存在，則回傳 `Some(FFmpegTask {...})`，
    /// 其中 `FFmpegTask` 是已經從內部 `tasks` 刪除的作業；
    /// 若作業不存在，則回傳 `None`。
    pub async fn abort_task(&mut self, ident: &str) -> FFmpegServiceResult<Option<FFmpegTask>> {
        let mut task = self.tasks.remove(ident);

        if let Some(ref mut task) = task {
            task.status = Interrupted(None);
            task.child.kill().await.map_err(FFmpegKillFailed)?;
        }

        Ok(task)
    }

    /// 列出所有登記在 FFmpegManager 的作業。
    pub fn list_all_tasks(&self) -> &HashMap<String, FFmpegTask> {
        &self.tasks
    }
}

type TaskParameter = FFmpegTaskParameters<'static>;

/// 更新作業的狀態。
fn update_status(task: &mut FFmpegTask) -> FFmpegServiceResult<()> {
    // Set the status to "Determining" to prevent the asynchronous
    // because of the following early return.
    task.status = FFmpegStatus::Determining;

    // Get the status.
    // The early return happened here.
    let status = task.child.try_wait().map_err(TryWaitFailed)?;

    // status == None -> command are still running
    //        == Some(...) -> command has finished
    if let Some(exit_status) = status {
        if exit_status.success() {
            task.status = FFmpegStatus::Completed;
        } else {
            task.status = FFmpegStatus::Interrupted(exit_status.code());
        }
    } else {
        task.status = FFmpegStatus::Running;
    }

    Ok(())
}
