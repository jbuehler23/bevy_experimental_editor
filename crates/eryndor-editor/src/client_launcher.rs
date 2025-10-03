use bevy::prelude::*;
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;

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
    /// Build the client executable if it doesn't exist
    fn ensure_client_built() -> Result<(), Box<dyn std::error::Error>> {
        info!("Building client executable...");

        let output = Command::new("cargo")
            .args(["build", "-p", "eryndor-client"])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to build client: {}", stderr).into());
        }

        info!("Client built successfully");
        Ok(())
    }

    /// Launch the client as a separate process
    pub fn launch(&mut self, project_path: PathBuf, level_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // Kill existing process if running
        self.stop();

        // Get the client executable path
        // In development, it's in target/debug/eryndor-client
        let exe_name = if cfg!(windows) {
            "eryndor-client.exe"
        } else {
            "eryndor-client"
        };

        // Build path from workspace root (go up from crates/eryndor-editor)
        let client_path = PathBuf::from("../../target/debug").join(exe_name);

        // Normalize the path
        let client_path = if client_path.exists() {
            client_path
        } else {
            // Try relative to current directory
            let alt_path = PathBuf::from("target").join("debug").join(exe_name);
            if alt_path.exists() {
                alt_path
            } else {
                // Build the client
                info!("Client executable not found, building...");
                Self::ensure_client_built()?;

                // Check again
                if alt_path.exists() {
                    alt_path
                } else if client_path.exists() {
                    client_path
                } else {
                    return Err(format!("Client executable still not found after build. Tried: {:?} and {:?}", client_path, alt_path).into());
                }
            }
        };

        info!("Using client executable at: {:?}", client_path);

        // Build command
        let mut cmd = Command::new(&client_path);
        cmd.arg("--project-path").arg(&project_path);

        if let Some(level) = level_path {
            cmd.arg("--level").arg(level);
        }

        // Spawn with output inherited so we can see logs
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        info!("Launching client with project: {:?}", project_path);

        match cmd.spawn() {
            Ok(child) => {
                info!("Client process started with PID: {:?}", child.id());
                self.process = Some(child);
                self.is_running = true;
                Ok(())
            }
            Err(e) => {
                error!("Failed to launch client: {}", e);
                Err(format!("Failed to launch client: {}", e).into())
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
