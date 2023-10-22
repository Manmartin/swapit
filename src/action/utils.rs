use std::fs::{self, File, OpenOptions};
use std::io::Result;
use std::path::Path;
pub struct TempFile<'a> {
    file: File,
    path: &'a Path,
}

impl<'a> TempFile<'a> {
    /// Create a new temp file from indicated path
    pub fn build(path: &'a Path) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        Ok(Self { file, path })
    }

    pub fn get_file(&'a self) -> &'a File {
        &self.file
    }
}

impl<'a> Drop for TempFile<'a> {
    fn drop(&mut self) {
        fs::remove_file(self.path).unwrap_or(());
    }
}
