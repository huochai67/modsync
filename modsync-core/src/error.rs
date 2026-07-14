use thiserror::Error;
use tokio::task::JoinError;

use crate::mstask::TaskEvent;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    StdIO(#[from] std::io::Error),

    #[error(transparent)]
    ZIPError(#[from] zip::result::ZipError),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    TokioSyncTrySend(#[from] tokio::sync::mpsc::error::TrySendError<TaskEvent>),

    #[error(transparent)]
    TokioSyncSend(#[from] tokio::sync::mpsc::error::SendError<TaskEvent>),

    #[error("tokio mpsc action failed.")]
    TokioMpscError,

    #[error("MSClientBuilder doesnt have msconfig")]
    BuilderNoMSConfig,

    #[error("function require msclient contain path")]
    MSClientNoPath,

    #[error("msconfig dont contain changelog url")]
    MSConfigNoChangeLogUrl,

    #[error("msconfig dont contain serverdat url")]
    MSConfigNoServerDatUrl,

    #[error("msconfig dont contain options url")]
    MSConfigNoOptionsUrl,

    #[error("msconfig dont contain hmcl url")]
    MSConfigNoHMCLUrl,

    #[error("msconfig dont contain pclce url")]
    MSConfigNoPCLCEUrl,

    #[error("msconfig dont contain modlist url")]
    MSConfigNoModListUrl,

    #[error("msconfig dont contain configpack")]
    MSConfigNoConfigPack,

    #[error("mpsc error")]
    MSTaskMPSC,

    #[error("invalid task: {0}")]
    InvalidTask(String),

    #[error("validation failed: {0}")]
    Validation(String),
}
