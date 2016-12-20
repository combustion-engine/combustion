use std::env;
use std::path::{Path, PathBuf};

pub fn resolve_asset_path<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::default()
}