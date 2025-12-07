use crate::config::Config;
use crate::scanner::{ProjectInfo, Scanner};
use std::fs;

pub struct StorageCleaner {
    pub projects: Vec<ProjectInfo>,
    pub config: Config,
    pub is_scanning: bool,
    pub status_message: String,
}

impl StorageCleaner {
    pub fn new() -> Self {
        let config = Config::load();
        let status_message = format!("Ready. Scan directory: {}", config.scan_path.display());

        Self {
            projects: Vec::new(),
            config,
            is_scanning: false,
            status_message,
        }
    }

    pub fn set_scan_path(&mut self, path: std::path::PathBuf) {
        self.config.scan_path = path;
        self.status_message = format!("Scan directory set to: {}", self.config.scan_path.display());

        // Save config
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save config: {}", e);
        }
    }

    pub fn scan_for_projects(&mut self) {
        self.is_scanning = true;
        self.projects.clear();
        self.status_message = format!("Scanning {} ...", self.config.scan_path.display());

        let scanner = Scanner::new(self.config.threshold_days);
        self.projects = scanner.scan(&self.config.scan_path);

        self.is_scanning = false;
        let total_size_gb: f64 = self.projects.iter().map(|p| p.size_gb()).sum();
        self.status_message = format!(
            "Found {} old project(s) - Total: {:.2} GB",
            self.projects.len(),
            total_size_gb
        );
    }

    pub fn delete_selected(&mut self) {
        let mut deleted_count = 0;
        let mut failed_count = 0;
        let mut freed_gb = 0.0;

        self.projects.retain(|project| {
            if project.selected {
                match fs::remove_dir_all(&project.node_modules_path) {
                    Ok(_) => {
                        deleted_count += 1;
                        freed_gb += project.size_gb();
                        false // Remove from list if deleted
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to delete {}: {}",
                            project.node_modules_path.display(),
                            e
                        );
                        failed_count += 1;
                        true // Keep in list if deletion failed
                    }
                }
            } else {
                true // Keep unselected projects
            }
        });

        self.status_message = if failed_count > 0 {
            format!(
                "✅ Deleted {} project(s) ({:.2} GB freed), ❌ {} failed",
                deleted_count, freed_gb, failed_count
            )
        } else {
            format!(
                "✅ Successfully deleted {} project(s) - Freed {:.2} GB",
                deleted_count, freed_gb
            )
        };
    }

    pub fn toggle_project(&mut self, index: usize) {
        if let Some(project) = self.projects.get_mut(index) {
            project.selected = !project.selected;
        }
    }

    pub fn total_selected_size_gb(&self) -> f64 {
        self.projects
            .iter()
            .filter(|p| p.selected)
            .map(|p| p.size_gb())
            .sum()
    }

    pub fn selected_count(&self) -> usize {
        self.projects.iter().filter(|p| p.selected).count()
    }
}
