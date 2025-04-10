pub mod backup;
pub mod link;
pub mod list;
pub mod unlink;

use std::path::PathBuf;

pub struct DotFile {
    pub source: PathBuf,
    pub target: PathBuf,
    pub profile: Option<String>,
}

impl DotFile {
    pub fn new(source: PathBuf, target: PathBuf, profile: Option<String>) -> Self {
        Self {
            source,
            target,
            profile,
        }
    }
}
