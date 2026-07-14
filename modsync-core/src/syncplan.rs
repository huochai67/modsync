use std::path::{Component, Path, PathBuf};

use crate::{
    error::Error,
    msclient::{DiffType, MODDiff},
    msmod::MSMOD,
    mstaskmanager::TaskRequest,
};

/// All filesystem locations used by one mod-sync installation.
/// Keeping this mapping in core prevents UI adapters from constructing unsafe paths.
#[derive(Debug, Clone)]
pub struct SyncPaths {
    root: PathBuf,
}

impl SyncPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn configpack_path(&self) -> PathBuf {
        self.root.join("configpack.zip")
    }

    pub fn configpack_extract_dir(&self) -> PathBuf {
        self.root.clone()
    }

    fn safe_relative_path(value: &str) -> Result<&Path, Error> {
        let path = Path::new(value);
        if path.is_absolute()
            || path.components().any(|part| {
                matches!(
                    part,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(Error::Validation(format!("unsafe mod path: {value}")));
        }
        Ok(path)
    }

    pub fn mod_path(&self, relative: &str) -> Result<PathBuf, Error> {
        Ok(self
            .root
            .join("mods")
            .join(Self::safe_relative_path(relative)?))
    }

    pub fn backup_path(&self, relative: &str) -> Result<PathBuf, Error> {
        Ok(self
            .root
            .join("bakmods")
            .join(Self::safe_relative_path(relative)?))
    }

    /// Translates an approved user selection into filesystem tasks. The planner is
    /// deliberately side-effect free, so callers can inspect it before execution.
    pub fn plan_diffs(&self, diffs: &[MODDiff], backup: bool) -> Result<Vec<TaskRequest>, Error> {
        let mut tasks = Vec::new();
        for diff in diffs {
            match diff.difftype {
                DiffType::NEWED | DiffType::MODIFIED => {
                    if let Some(remote) = &diff.remote {
                        let url = remote.url.clone().ok_or_else(|| {
                            Error::Validation(format!("mod {} has no download URL", remote.path))
                        })?;
                        tasks.push(TaskRequest::download_verified(
                            format!("下载{}", remote.path),
                            url,
                            self.mod_path(&remote.path)?.to_string_lossy().to_string(),
                            Some(remote.md5.clone()),
                        ));
                    }
                }
                DiffType::DELETED => {
                    if let Some(local) = &diff.local {
                        let source = self.mod_path(&local.path)?.to_string_lossy().to_string();
                        if backup {
                            tasks.push(TaskRequest::rename(
                                format!("删除{}", local.path),
                                source,
                                self.backup_path(&local.path)?.to_string_lossy().to_string(),
                            ));
                        } else {
                            tasks.push(TaskRequest::delete(format!("删除{}", local.path), source));
                        }
                    }
                }
            }
        }
        Ok(tasks)
    }

    pub fn plan_configpack_download(
        &self,
        configpack: &MSMOD,
    ) -> Result<Option<TaskRequest>, Error> {
        let target = self.configpack_path();
        if target.exists() && MSMOD::from_file(&target, "", None)?.md5 == configpack.md5 {
            return Ok(None);
        }
        let url = configpack
            .url
            .clone()
            .ok_or_else(|| Error::Validation("config pack has no download URL".to_string()))?;
        Ok(Some(TaskRequest::download_verified(
            "Download ConfigPack".to_string(),
            url,
            target.to_string_lossy().to_string(),
            Some(configpack.md5.clone()),
        )))
    }

    pub fn configpack_extract_task(&self) -> TaskRequest {
        TaskRequest::unzip(
            "Process ConfigPack".to_string(),
            self.configpack_path().to_string_lossy().to_string(),
            self.configpack_extract_dir().to_string_lossy().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_reject_traversal() {
        let paths = SyncPaths::new("minecraft");
        assert!(paths.mod_path("../outside.jar").is_err());
        assert!(paths.backup_path("C:/outside.jar").is_err());
    }
}
