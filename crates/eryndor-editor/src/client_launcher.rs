use bevy::prelude::*;
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;
use crate::project_generator::get_package_name_from_cargo_toml;

/// Resource to manage the standalone client process
#[derive(Resource)]
pub struct StandaloneClient {
    pub process: Option<Child>,
    pub is_running: bool,
}

impl Default for StandaloneClient {
    fn default() -> Self {
        Self {
            process: None,
            is_running: false,
        }
    }
}

impl StandaloneClient {
    /// Build the project's client executable
    fn build_project(project_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        info!("Building project at {:?}...", project_path);
        info!("This may take a few minutes for the first build...");

        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(format!("No Cargo.toml found at {:?}", cargo_toml).into());
        }

        // Use spawn + wait with inherited stdio to show build progress in real-time
        let status = Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(&cargo_toml)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err("Failed to build project (see output above)".into());
        }

        info!("Project built successfully!");
        Ok(())
    }

    /// Launch the client as a separate process
    pub fn launch(&mut self, project_path: PathBuf, level_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // Kill existing process if running
        self.stop();

        let cargo_toml = project_path.join("Cargo.toml");

        info!("Launching game with cargo run from: {:?}", project_path);

        // Use cargo run instead of direct exe - this ensures we always have latest build
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
           .arg("--manifest-path")
           .arg(&cargo_toml)
           .arg("--");  // Separator between cargo args and binary args

        // Pass project path and level path as arguments to the binary
        cmd.arg("--project-path").arg(&project_path);

        if let Some(level) = level_path {
            cmd.arg("--level").arg(level);
        }

        // Spawn with output inherited so we can see logs
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        match cmd.spawn() {
            Ok(child) => {
                info!("Process started with PID: {:?}", child.id());
                self.process = Some(child);
                self.is_running = true;
                Ok(())
            }
            Err(e) => {
                error!("Failed to launch project: {}", e);
                Err(format!("Failed to launch project: {}", e).into())
            }
        }
    }

    /// Stop the running client process
    pub fn stop(&mut self) {
        if let Some(mut process) = self.process.take() {
            match process.kill() {
                Ok(_) => info!("Client process terminated"),
                Err(e) => error!("Failed to kill client process: {}", e),
            }
            self.is_running = false;
        }
    }

    /// Check if client is still running
    pub fn check_status(&mut self) {
        if let Some(ref mut process) = self.process {
            match process.try_wait() {
                Ok(Some(status)) => {
                    info!("Client process exited with status: {}", status);
                    self.process = None;
                    self.is_running = false;
                }
                Ok(None) => {
                    // Still running
                }
                Err(e) => {
                    error!("Error checking client process: {}", e);
                    self.process = None;
                    self.is_running = false;
                }
            }
        }
    }
}

/// System to monitor client process status
pub fn monitor_client_process(mut client: ResMut<StandaloneClient>) {
    client.check_status();
}

/// System to clean up client process on editor exit
pub fn cleanup_client_on_exit(mut client: ResMut<StandaloneClient>) {
    client.stop();
}
