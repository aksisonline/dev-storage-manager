use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct ProjectInfo {
    pub project_path: PathBuf,
    pub node_modules_path: PathBuf,
    pub last_modified: SystemTime,
    pub size_mb: f64,
    pub selected: bool,
}

impl ProjectInfo {
    pub fn days_old(&self) -> u64 {
        if let Ok(duration) = SystemTime::now().duration_since(self.last_modified) {
            duration.as_secs() / 86400
        } else {
            0
        }
    }

    pub fn size_gb(&self) -> f64 {
        self.size_mb / 1024.0
    }
}

pub struct Scanner {
    threshold_days: u32,
}

impl Scanner {
    pub fn new(threshold_days: u32) -> Self {
        Self { threshold_days }
    }

    pub fn scan(&self, scan_path: &Path) -> Vec<ProjectInfo> {
        let mut projects = Vec::new();

        for entry in WalkDir::new(scan_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip hidden directories and common non-project directories
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "Library" || name == "System" {
                    continue;
                }
            }

            if path.file_name() == Some(std::ffi::OsStr::new("node_modules")) && path.is_dir() {
                if let Some(project_path) = path.parent() {
                    if let Ok(metadata) = fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            let days_old =
                                if let Ok(duration) = SystemTime::now().duration_since(modified) {
                                    duration.as_secs() / 86400
                                } else {
                                    0
                                };

                            if days_old >= self.threshold_days as u64 {
                                let size = Self::calculate_dir_size(path);
                                let size_mb = size as f64 / (1024.0 * 1024.0);

                                projects.push(ProjectInfo {
                                    project_path: project_path.to_path_buf(),
                                    node_modules_path: path.to_path_buf(),
                                    last_modified: modified,
                                    size_mb,
                                    selected: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort by size (largest first)
        projects.sort_by(|a, b| {
            b.size_mb
                .partial_cmp(&a.size_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        projects
    }

    fn calculate_dir_size(path: &Path) -> u64 {
        let mut size = 0u64;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        size += metadata.len();
                    } else if metadata.is_dir() {
                        size += Self::calculate_dir_size(&entry.path());
                    }
                }
            }
        }
        size
    }
}
