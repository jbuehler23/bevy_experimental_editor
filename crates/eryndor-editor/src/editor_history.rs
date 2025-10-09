//! Undo/Redo system for the editor
//!
//! Implements a command pattern with full history tracking for all editor operations.
//! Supports unlimited undo/redo, command merging, and history panel visualization.

use bevy::prelude::*;
use std::collections::VecDeque;

/// Maximum number of commands to keep in history (prevents unbounded memory growth)
const MAX_HISTORY_SIZE: usize = 100;

/// Trait for all undoable editor commands
pub trait EditorCommand: Send + Sync {
    /// Execute the command (do the action)
    fn execute(&mut self, world: &mut World);

    /// Undo the command (reverse the action)
    fn undo(&mut self, world: &mut World);

    /// Get a human-readable description for the history panel
    fn description(&self) -> String;

    /// Whether this command can be merged with another command of the same type
    /// Used for continuous operations like dragging
    fn can_merge_with(&self, _other: &dyn EditorCommand) -> bool {
        false
    }

    /// Merge this command with another (for continuous operations)
    fn merge(&mut self, _other: Box<dyn EditorCommand>) {
        // Default: do nothing
    }
}

/// Wrapper to make commands object-safe
struct CommandWrapper {
    command: Box<dyn EditorCommand>,
    /// Whether this command has been executed
    executed: bool,
}

/// Resource that manages undo/redo history
#[derive(Resource)]
pub struct EditorHistory {
    /// Stack of commands that have been executed (can be undone)
    undo_stack: VecDeque<CommandWrapper>,

    /// Stack of commands that have been undone (can be redone)
    redo_stack: VecDeque<CommandWrapper>,

    /// Whether we're currently executing/undoing a command (prevents recursion)
    is_executing: bool,

    /// Number of commands executed this session (for statistics)
    total_commands: usize,
}

impl Default for EditorHistory {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            is_executing: false,
            total_commands: 0,
        }
    }
}

impl EditorHistory {
    /// Execute a new command and add it to the history
    pub fn execute(&mut self, mut command: Box<dyn EditorCommand>, world: &mut World) {
        if self.is_executing {
            warn!("Attempted to execute command while already executing - ignoring");
            return;
        }

        self.is_executing = true;

        // Execute the command
        command.execute(world);

        // Clear redo stack when new command is executed
        self.redo_stack.clear();

        // Try to merge with last command if possible
        if let Some(last) = self.undo_stack.back_mut() {
            if last.command.can_merge_with(command.as_ref()) {
                last.command.merge(command);
                self.is_executing = false;
                return;
            }
        }

        // Add to undo stack
        self.undo_stack.push_back(CommandWrapper {
            command,
            executed: true,
        });

        // Limit history size
        if self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }

        self.total_commands += 1;
        self.is_executing = false;

        info!("Executed command (history size: {})", self.undo_stack.len());
    }

    /// Add a command to history without executing it (for commands already applied)
    pub fn add_executed(&mut self, command: Box<dyn EditorCommand>) {
        if self.is_executing {
            warn!("Attempted to add command while executing - ignoring");
            return;
        }

        // Clear redo stack when new command is added
        self.redo_stack.clear();

        // Try to merge with last command if possible
        if let Some(last) = self.undo_stack.back_mut() {
            if last.command.can_merge_with(command.as_ref()) {
                last.command.merge(command);
                return;
            }
        }

        // Add to undo stack (already executed)
        self.undo_stack.push_back(CommandWrapper {
            command,
            executed: true,
        });

        // Limit history size
        if self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }

        self.total_commands += 1;

        info!("Added executed command to history (size: {})", self.undo_stack.len());
    }

    /// Undo the last command
    pub fn undo(&mut self, world: &mut World) -> bool {
        if self.is_executing {
            warn!("Attempted to undo while executing command - ignoring");
            return false;
        }

        if let Some(mut wrapper) = self.undo_stack.pop_back() {
            self.is_executing = true;

            let desc = wrapper.command.description();
            wrapper.command.undo(world);
            wrapper.executed = false;

            self.redo_stack.push_back(wrapper);
            self.is_executing = false;

            info!("Undid: {}", desc);
            true
        } else {
            false
        }
    }

    /// Redo the last undone command
    pub fn redo(&mut self, world: &mut World) -> bool {
        if self.is_executing {
            warn!("Attempted to redo while executing command - ignoring");
            return false;
        }

        if let Some(mut wrapper) = self.redo_stack.pop_back() {
            self.is_executing = true;

            let desc = wrapper.command.description();
            wrapper.command.execute(world);
            wrapper.executed = true;

            self.undo_stack.push_back(wrapper);
            self.is_executing = false;

            info!("Redid: {}", desc);
            true
        } else {
            false
        }
    }

    /// Check if there are commands to undo
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if there are commands to redo
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the description of the next command to undo
    pub fn undo_description(&self) -> Option<String> {
        self.undo_stack.back().map(|w| w.command.description())
    }

    /// Get the description of the next command to redo
    pub fn redo_description(&self) -> Option<String> {
        self.redo_stack.back().map(|w| w.command.description())
    }

    /// Get all commands in the undo stack (for history panel)
    pub fn get_undo_history(&self) -> Vec<String> {
        self.undo_stack
            .iter()
            .map(|w| w.command.description())
            .collect()
    }

    /// Get all commands in the redo stack (for history panel)
    pub fn get_redo_history(&self) -> Vec<String> {
        self.redo_stack
            .iter()
            .rev()  // Reverse to show in chronological order
            .map(|w| w.command.description())
            .collect()
    }

    /// Clear all history (e.g., when loading a new scene)
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        info!("Cleared editor history");
    }

    /// Get statistics about the history
    pub fn stats(&self) -> HistoryStats {
        HistoryStats {
            undo_count: self.undo_stack.len(),
            redo_count: self.redo_stack.len(),
            total_commands: self.total_commands,
        }
    }
}

/// Statistics about the editor history
#[derive(Debug, Clone, Copy)]
pub struct HistoryStats {
    pub undo_count: usize,
    pub redo_count: usize,
    pub total_commands: usize,
}

/// System to handle keyboard shortcuts for undo/redo
pub fn handle_undo_redo_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut history: ResMut<EditorHistory>,
    world: &mut World,
) {
    // Ctrl+Z - Undo
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        if keyboard.just_pressed(KeyCode::KeyZ) {
            // Ctrl+Shift+Z - Redo (alternative)
            if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                history.redo(world);
            } else {
                // Ctrl+Z - Undo
                history.undo(world);
            }
        }
        // Ctrl+Y - Redo (Windows-style)
        else if keyboard.just_pressed(KeyCode::KeyY) {
            history.redo(world);
        }
    }
}
