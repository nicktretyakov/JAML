use crate::core::{CmsResult, StaticWriter};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileSystemWriter {
    root: PathBuf,
}

impl FileSystemWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let target = dest.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

impl StaticWriter for FileSystemWriter {
    fn write_file(&self, relative_path: &str, content: &str) -> CmsResult<()> {
        let full = self.root.join(relative_path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full, content)?;
        Ok(())
    }

    fn copy_static_dir(&self, src: &Path, dest: &Path) -> CmsResult<()> {
        copy_dir_recursive(src, dest)?;
        Ok(())
    }

    fn ensure_dir(&self, path: &Path) -> CmsResult<()> {
        fs::create_dir_all(self.root.join(path))?;
        Ok(())
    }
}
