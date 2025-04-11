use std::io;
use std::path::{Path, PathBuf};

pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(source: P, target: Q) -> io::Result<()> {
    // Implementation of create_symlink
    #[cfg(unix)]
    return std::os::unix::fs::symlink(source, target);
    
    #[cfg(windows)]
    {
        let source = source.as_ref();
        if source.is_dir() {
            return std::os::windows::fs::symlink_dir(source, target);
        } else {
            return std::os::windows::fs::symlink_file(source, target);
        }
    }
}

pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_symlink()
}

pub fn get_symlink_target<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    if !path.is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a symlink",
        ));
    }
    
    std::fs::read_link(path)
}
