use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};

/// Status of the build process
#[derive(Debug, Clone)]
pub enum BuildStatus {
    Idle,
    Building { stage: String, elapsed_secs: f32 },
    Finished { duration_secs: f32 },
    Failed { error: String },
}

/// Resource to manage async project builds
#[derive(Resource)]
pub struct BuildManager {
    build_thread: Option<JoinHandle<()>>,
    status_tx: Option<Sender<BuildStatus>>,
    status_rx: Receiver<BuildStatus>,
    pub current_status: BuildStatus,
    start_time: Option<std::time::Instant>,
}

impl Default for BuildManager {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self {
            build_thread: None,
            status_tx: Some(tx),
            status_rx: rx,
            current_status: BuildStatus::Idle,
            start_time: None,
        }
    }
}

impl BuildManager {
    /// Start building a project asynchronously
    pub fn start_build(&mut self, project_path: PathBuf, package_name: String) {
        // Cancel any existing build
        self.cancel_build();

        let (tx, rx) = unbounded();
        self.status_tx = Some(tx.clone());
        self.status_rx = rx;
        self.start_time = Some(std::time::Instant::now());
        self.current_status = BuildStatus::Building {
            stage: "Starting build...".to_string(),
            elapsed_secs: 0.0,
        };

        info!("Starting async build for project: {}", package_name);

        // Spawn build thread
        let handle = thread::spawn(move || {
            let cargo_toml = project_path.join("Cargo.toml");

            tx.send(BuildStatus::Building {
                stage: format!("Building {}...", package_name),
                elapsed_secs: 0.0,
            })
            .ok();

            // Run cargo build
            let result = Command::new("cargo")
                .arg("build")
                .arg("--manifest-path")
                .arg(&cargo_toml)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match result {
                Ok(status) if status.success() => {
                    info!("Build completed successfully");
                    tx.send(BuildStatus::Finished { duration_secs: 0.0 })
                        .ok();
                }
                Ok(_) => {
                    error!("Build failed");
                    tx.send(BuildStatus::Failed {
                        error: "Build failed (see console output)".to_string(),
                    })
                    .ok();
                }
                Err(e) => {
                    error!("Failed to run cargo build: {}", e);
                    tx.send(BuildStatus::Failed {
                        error: format!("Failed to run cargo: {}", e),
                    })
                    .ok();
                }
            }
        });

        self.build_thread = Some(handle);
    }

    /// Cancel the current build
    pub fn cancel_build(&mut self) {
        if let Some(handle) = self.build_thread.take() {
            info!("Canceling build...");
            // Note: This doesn't actually kill the cargo process, just detaches from it
            // To properly kill it, we'd need to store the child process handle
            drop(handle);
            self.current_status = BuildStatus::Idle;
            self.start_time = None;
        }
    }

    /// Check if a build is currently running
    pub fn is_building(&self) -> bool {
        matches!(self.current_status, BuildStatus::Building { .. })
    }

    /// Get the current build stage description
    pub fn current_stage(&self) -> String {
        match &self.current_status {
            BuildStatus::Idle => "Ready".to_string(),
            BuildStatus::Building { stage, .. } => stage.clone(),
            BuildStatus::Finished { .. } => "Build complete!".to_string(),
            BuildStatus::Failed { error } => format!("Build failed: {}", error),
        }
    }

    /// Get elapsed build time
    pub fn elapsed_time(&self) -> f32 {
        if let Some(start) = self.start_time {
            start.elapsed().as_secs_f32()
        } else {
            0.0
        }
    }
}

/// System to poll build status from background thread
pub fn poll_build_status(mut build_manager: ResMut<BuildManager>) {
    // Try to receive status updates from the build thread
    while let Ok(status) = build_manager.status_rx.try_recv() {
        match &status {
            BuildStatus::Finished { .. } => {
                let duration = build_manager.elapsed_time();
                build_manager.current_status = BuildStatus::Finished {
                    duration_secs: duration,
                };
                build_manager.start_time = None;
                info!("Build finished in {:.1}s", duration);
            }
            BuildStatus::Failed { error } => {
                error!("Build failed: {}", error);
                build_manager.current_status = status;
                build_manager.start_time = None;
            }
            BuildStatus::Building { stage, .. } => {
                let elapsed = build_manager.elapsed_time();
                build_manager.current_status = BuildStatus::Building {
                    stage: stage.clone(),
                    elapsed_secs: elapsed,
                };
            }
            BuildStatus::Idle => {
                build_manager.current_status = BuildStatus::Idle;
                build_manager.start_time = None;
            }
        }
    }

    // Update elapsed time if building
    if let BuildStatus::Building { stage, .. } = &build_manager.current_status {
        let elapsed = build_manager.elapsed_time();
        build_manager.current_status = BuildStatus::Building {
            stage: stage.clone(),
            elapsed_secs: elapsed,
        };
    }
}
