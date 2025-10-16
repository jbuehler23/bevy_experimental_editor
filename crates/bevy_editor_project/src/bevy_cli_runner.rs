use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;

/// Manages running bevy CLI commands from within the editor
#[derive(Resource)]
pub struct BevyCLIRunner {
    /// Path to the current project directory
    pub project_path: Option<PathBuf>,
    /// Currently running process (if any)
    pub running_process: Option<RunningProcess>,
    /// Channel for receiving command output
    output_receiver: Receiver<CLIOutput>,
    /// Sender for command output (used internally)
    output_sender: Sender<CLIOutput>,
    /// Accumulated output lines for display
    pub output_lines: Vec<CLIOutputLine>,
    /// Maximum number of output lines to keep
    pub max_output_lines: usize,
}

/// A running CLI process
pub struct RunningProcess {
    pub command: CLICommand,
    pub child: Child,
    pub start_time: std::time::Instant,
}

/// Available bevy CLI commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLICommand {
    /// Run native dev build (bevy run)
    Run,
    /// Run web dev build (bevy run web)
    RunWeb,
    /// Build native dev (cargo build)
    Build,
    /// Build web bundle (bevy build web --bundle)
    BuildWeb,
    /// Run linter (bevy lint)
    Lint,
    /// Build release (cargo build --release)
    BuildRelease,
}

impl CLICommand {
    pub fn name(&self) -> &str {
        match self {
            CLICommand::Run => "Run Game",
            CLICommand::RunWeb => "Run Web",
            CLICommand::Build => "Build",
            CLICommand::BuildWeb => "Build Web",
            CLICommand::Lint => "Lint",
            CLICommand::BuildRelease => "Build Release",
        }
    }

    pub fn command_args(&self) -> (&str, Vec<&str>) {
        match self {
            CLICommand::Run => ("bevy", vec!["run"]),
            CLICommand::RunWeb => ("bevy", vec!["run", "web"]),
            CLICommand::Build => ("cargo", vec!["build"]),
            CLICommand::BuildWeb => ("bevy", vec!["build", "web", "--bundle"]),
            CLICommand::Lint => ("bevy", vec!["lint"]),
            CLICommand::BuildRelease => ("cargo", vec!["build", "--release"]),
        }
    }

    pub fn is_long_running(&self) -> bool {
        matches!(self, CLICommand::Run | CLICommand::RunWeb)
    }
}

/// Output from a CLI command
#[derive(Debug, Clone)]
pub enum CLIOutput {
    Stdout(String),
    Stderr(String),
    Exit(i32),
}

/// A line of CLI output with metadata
#[derive(Debug, Clone)]
pub struct CLIOutputLine {
    pub text: String,
    pub is_error: bool,
    pub timestamp: f64,
}

impl Default for BevyCLIRunner {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            project_path: None,
            running_process: None,
            output_receiver: receiver,
            output_sender: sender,
            output_lines: Vec::new(),
            max_output_lines: 1000,
        }
    }
}

impl BevyCLIRunner {
    /// Set the current project path
    pub fn set_project_path(&mut self, path: PathBuf) {
        self.project_path = Some(path);
    }

    /// Check if a command is currently running
    pub fn is_running(&self) -> bool {
        self.running_process.is_some()
    }

    /// Get the currently running command, if any
    pub fn current_command(&self) -> Option<CLICommand> {
        self.running_process.as_ref().map(|p| p.command)
    }

    /// Start running a CLI command
    pub fn run_command(&mut self, command: CLICommand) -> Result<(), String> {
        // Check if we have a project path and clone it
        let project_path = self
            .project_path
            .as_ref()
            .ok_or("No project loaded")?
            .clone();

        // Stop any currently running process
        if self.is_running() {
            self.stop_current_process();
        }

        // Clear previous output
        self.output_lines.clear();

        let (cmd, args) = command.command_args();

        // Spawn the process
        let mut child = Command::new(cmd)
            .args(&args)
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", cmd, e))?;

        // Capture stdout in a separate thread
        let stdout = child.stdout.take().unwrap();
        let sender = self.output_sender.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                let _ = sender.send(CLIOutput::Stdout(line));
            }
        });

        // Capture stderr in a separate thread
        let stderr = child.stderr.take().unwrap();
        let sender = self.output_sender.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                let _ = sender.send(CLIOutput::Stderr(line));
            }
        });

        self.running_process = Some(RunningProcess {
            command,
            child,
            start_time: std::time::Instant::now(),
        });

        Ok(())
    }

    /// Stop the currently running process
    pub fn stop_current_process(&mut self) {
        if let Some(mut process) = self.running_process.take() {
            let _ = process.child.kill();
            let _ = process.child.wait();
        }
    }

    /// Process any new output from the running command
    pub fn update(&mut self, time: f64) {
        // Check if process has exited
        if let Some(process) = &mut self.running_process {
            if let Ok(Some(status)) = process.child.try_wait() {
                let exit_code = status.code().unwrap_or(-1);
                let _ = self.output_sender.send(CLIOutput::Exit(exit_code));
                self.running_process = None;
            }
        }

        // Process any pending output
        while let Ok(output) = self.output_receiver.try_recv() {
            match output {
                CLIOutput::Stdout(line) => {
                    self.output_lines.push(CLIOutputLine {
                        text: line,
                        is_error: false,
                        timestamp: time,
                    });
                }
                CLIOutput::Stderr(line) => {
                    self.output_lines.push(CLIOutputLine {
                        text: line,
                        is_error: true,
                        timestamp: time,
                    });
                }
                CLIOutput::Exit(code) => {
                    let msg = if code == 0 {
                        format!("Process exited successfully (code: {})", code)
                    } else {
                        format!("Process exited with error (code: {})", code)
                    };
                    self.output_lines.push(CLIOutputLine {
                        text: msg,
                        is_error: code != 0,
                        timestamp: time,
                    });
                }
            }
        }

        // Trim output if it exceeds max lines
        if self.output_lines.len() > self.max_output_lines {
            let excess = self.output_lines.len() - self.max_output_lines;
            self.output_lines.drain(0..excess);
        }
    }

    /// Clear all output
    pub fn clear_output(&mut self) {
        self.output_lines.clear();
    }
}

/// System to update CLI runner
pub fn update_cli_runner(mut cli_runner: ResMut<BevyCLIRunner>, time: Res<Time>) {
    cli_runner.update(time.elapsed_secs_f64());
}
