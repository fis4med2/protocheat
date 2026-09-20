use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NtsyncError {
    #[error("device not found at {0}")]
    Missing(PathBuf),
    #[error("permission denied on {0}")]
    Permission(PathBuf),
    #[error("io error: {0}")]
    Io(String),
}

#[derive(Debug, Clone)]
pub struct NtsyncStatus {
    pub path: PathBuf,
    pub present: bool,
    pub readable: bool,
    pub writable: bool,
}

pub fn device_path() -> PathBuf {
    std::env::var("PROTOCHEAT_NTSYNC")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("/dev/ntsync"))
}

pub fn check() -> Result<NtsyncStatus, NtsyncError> {
    let path = device_path();
    if !path.exists() {
        return Err(NtsyncError::Missing(path));
    }
    let readable = std::fs::File::open(&path).is_ok();
    let writable = std::fs::OpenOptions::new().write(true).open(&path).is_ok();
    if !readable {
        return Err(NtsyncError::Permission(path));
    }
    Ok(NtsyncStatus {
        path,
        present: true,
        readable,
        writable,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_returns_struct_or_missing() {
        match check() {
            Ok(s) => assert!(s.present),
            Err(NtsyncError::Missing(_)) => {}
            Err(NtsyncError::Permission(_)) => {}
            Err(e) => panic!("unexpected: {e}"),
        }
    }
}
