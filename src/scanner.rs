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
        self.scan_with_progress(scan_path, |_folder, _progress| {})
    }

    pub fn scan_with_progress<F>(
        &self,
        scan_path: &Path,
        mut progress_callback: F,
    ) -> Vec<ProjectInfo>
    where
        F: FnMut(&str, f32),
    {
        let mut projects = Vec::new();
        let mut processed_count = 0;

        for entry in WalkDir::new(scan_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Skip hidden directories and common non-project directories
                if let Some(name) = e.file_name().to_str() {
                    // Skip hidden files/dirs
                    if name.starts_with('.') {
                        return false;
                    }
                    // Skip system directories
                    if name == "Library" || name == "System" {
                        return false;
                    }
                }
                true
            })
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Update progress periodically
            processed_count += 1;
            if processed_count % 10 == 0 {
                let folder_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                progress_callback(folder_name, 0.0); // We don't have total count, just report activity
            }

            // Skip anything that's already inside a node_modules directory (check all ancestors)
            // This prevents recursing into nested node_modules like:
            // /project/node_modules/@esbuild-kit/core-utils/node_modules
            // We still want to detect the top-level /project/node_modules
            let mut has_node_modules_ancestor = false;
            for ancestor in path.ancestors().skip(1) {
                if ancestor.file_name() == Some(std::ffi::OsStr::new("node_modules")) {
                    has_node_modules_ancestor = true;
                    break;
                }
            }
            if has_node_modules_ancestor {
                continue;
            }

            // Now check if this entry IS a node_modules directory at the project level
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
