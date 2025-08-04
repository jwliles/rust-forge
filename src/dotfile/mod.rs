pub mod backup;
pub mod link;
pub mod list;
pub mod unlink;

use std::path::PathBuf;

pub struct DotFile {
    pub source: PathBuf,
    pub target: PathBuf,
    pub profile: Option<String>,
    pub status: DotFileStatus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DotFileStatus {
    Staged,
    Linked,
    Unlinked,
}

impl DotFile {
    pub fn new(source: PathBuf, target: PathBuf, profile: Option<String>) -> Self {
        Self {
            source,
            target,
            profile,
            status: DotFileStatus::Staged,
        }
    }

    pub fn with_status(
        source: PathBuf,
        target: PathBuf,
        profile: Option<String>,
        status: DotFileStatus,
    ) -> Self {
        Self {
            source,
            target,
            profile,
            status,
        }
    }

    pub fn set_status(&mut self, status: DotFileStatus) {
        self.status = status;
    }

    pub fn is_staged(&self) -> bool {
        self.status == DotFileStatus::Staged
    }

    pub fn is_linked(&self) -> bool {
        self.status == DotFileStatus::Linked
    }

    pub fn is_unlinked(&self) -> bool {
        self.status == DotFileStatus::Unlinked
    }
}
