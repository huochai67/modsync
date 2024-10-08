use thiserror::Error;

use crate::mstask::MSTaskStatus;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    StdIO(#[from] std::io::Error),

    #[error(transparent)]
    TokioSyncTrySend(#[from] tokio::sync::mpsc::error::TrySendError<MSTaskStatus>),

    #[error("MSClientBuilder doesnt have msconfig")]
    BuilderNoMSConfig,

    #[error("function require msclient contain path")]
    MSClientNoPath,

    #[error("msconfig dont contain changelog url")]
    MSConfigNoChangeLogUrl,

    #[error("msconfig dont contain serverlist url")]
    MSConfigNoServerListUrl,

    #[error("msconfig dont contain optionlist url")]
    MSConfigNoOptionListUrl,

    #[error("msconfig dont contain modlist url")]
    MSConfigNoModListUrl,

    #[error("msconfig dont contain modlist url")]
    MSConfigNoNecessaryListUrl,

    #[error("mpsc error")]
    MSTaskMPSC,
}
