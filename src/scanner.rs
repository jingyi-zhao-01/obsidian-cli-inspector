use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub mtime: u64,
    pub size: u64,
}

pub struct VaultScanner {
    vault_path: PathBuf,
    exclude_patterns: Vec<String>,
}

impl VaultScanner {
    pub fn new(vault_path: PathBuf, exclude_patterns: Vec<String>) -> Self {
        VaultScanner {
            vault_path,
            exclude_patterns,
        }
    }

    pub fn scan(&self) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        self.walk_dir(&self.vault_path, &mut entries)?;
        Ok(entries)
    }

    fn walk_dir(&self, dir: &Path, entries: &mut Vec<FileEntry>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&self.vault_path)?.to_path_buf();

            // Check if should exclude
            if self.should_exclude(&relative_path) {
                continue;
            }

            if path.is_dir() {
                self.walk_dir(&path, entries)?;
            } else if path.is_file() {
                // Only index markdown files
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let metadata = fs::metadata(&path)?;
                        let mtime = metadata
                            .modified()?
                            .duration_since(std::time::UNIX_EPOCH)?
                            .as_secs();

                        entries.push(FileEntry {
                            path,
                            relative_path: relative_path.to_string_lossy().to_string(),
                            mtime,
                            size: metadata.len(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        false
    }
}
