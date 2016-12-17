use std::fs::{DirEntry, read_dir};
use std::path::Path;

/// Recursively visit directories
pub fn visit_dirs<F>(dir: &Path, cb: &F) where F: Fn(&DirEntry){
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() { visit_dirs(&path, cb); } else { cb(&entry); }
        }
    }
}