use crate::config::Config;
use crate::scanner::{ProjectInfo, Scanner};
use std::fs;

pub struct StorageCleaner {
    pub projects: Vec<ProjectInfo>,
    pub all_projects: Vec<ProjectInfo>, // Cache of all scanned projects
    pub config: Config,
    pub is_scanning: bool,
    pub status_message: String,
    pub scan_progress: f32,
    pub current_scan_folder: String,
    pub threshold_enabled: bool,
}

impl StorageCleaner {
    pub fn new() -> Self {
        let config = Config::load();
        let status_message = format!("Ready. Scan directory: {}", config.scan_path.display());

        Self {
            projects: Vec::new(),
            all_projects: Vec::new(),
            config,
            is_scanning: false,
            status_message,
            scan_progress: 0.0,
            current_scan_folder: String::new(),
            threshold_enabled: true,
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
        if self.is_scanning {
            return; // Prevent multiple scans
        }

        self.is_scanning = true;
        self.projects.clear();
        self.all_projects.clear();
        self.scan_progress = 0.0;
        self.current_scan_folder = String::new();
        self.status_message = format!("Scanning {} ...", self.config.scan_path.display());

        // Scan ALL projects regardless of threshold
        // Note: Synchronous for now - async implementation causes type inference issues with GPUI
        let scanner = Scanner::new(0);
        self.all_projects = scanner.scan(&self.config.scan_path);

        self.is_scanning = false;
        self.scan_progress = 1.0;
        self.current_scan_folder.clear();

        // Apply filter based on current threshold setting
        self.apply_filter();
    }

    pub fn apply_filter(&mut self) {
        if self.threshold_enabled {
            self.projects = self
                .all_projects
                .iter()
                .filter(|p| p.days_old() >= self.config.threshold_days as u64)
                .cloned()
                .collect();
        } else {
            self.projects = self.all_projects.clone();
        }

        let total_size_gb: f64 = self.projects.iter().map(|p| p.size_gb()).sum();
        self.status_message = format!(
            "Found {} node_modules folder(s) - Total: {:.2} GB",
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
                "✅ Deleted {} node_modules ({:.2} GB freed), ❌ {} failed",
                deleted_count, freed_gb, failed_count
            )
        } else {
            format!(
                "✅ Successfully deleted {} node_modules - Freed {:.2} GB",
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

    pub fn increase_threshold(&mut self) {
        if self.config.threshold_days < 365 {
            self.config.threshold_days += 1;
            let _ = self.config.save();
            if !self.all_projects.is_empty() {
                self.apply_filter();
            }
        }
    }

    pub fn decrease_threshold(&mut self) {
        if self.config.threshold_days > 0 {
            self.config.threshold_days -= 1;
            let _ = self.config.save();
            if !self.all_projects.is_empty() {
                self.apply_filter();
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_threshold(&mut self, days: u32) {
        if days <= 365 {
            self.config.threshold_days = days;
            let _ = self.config.save();
            if !self.all_projects.is_empty() {
                self.apply_filter();
            }
        }
    }

    pub fn toggle_threshold(&mut self) {
        self.threshold_enabled = !self.threshold_enabled;
        if !self.all_projects.is_empty() {
            self.apply_filter();
        }
    }
}
