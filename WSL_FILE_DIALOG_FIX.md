# File Dialog Fix for WSL/Linux

## Problem
When running the Bevy editor in WSL (Windows Subsystem for Linux), the native file dialog fails with errors:
```
ERROR rfd::backend::xdg_desktop_portal: Failed to pick folder: Portal request failed
ERROR rfd::backend::xdg_desktop_portal: Failed to pick folder with zenity: Permission denied
```

This is because WSL doesn't have the necessary display server components for native file dialogs.

## Solution Implemented

We've added a **fallback manual path entry system** that automatically activates when file dialogs fail.

### Features Added

1. **Automatic Fallback**: When the file dialog fails (common in WSL), the UI automatically shows a manual path entry field
2. **Manual Entry Button**: Added a "✏ Manual" button to explicitly switch to manual entry mode
3. **Path Validation**: Real-time validation shows if the path exists and is valid
4. **Create & Use**: Option to create directories that don't exist yet
5. **Helpful Tips**: Contextual tips for WSL users explaining the issue

### Usage

#### Opening a Project
1. Click "📂 Open Existing Project"
2. If the dialog fails, you'll see a text input field
3. Enter your project path (e.g., `/home/user/projects/my_game`)
4. Click "Use This Path" or "Create & Use"

#### Creating a New Project
1. Click "📁 Create New Project"
2. Enter your project name
3. Click "Browse..." for the location
4. If it fails, you'll see manual entry
5. You can also click "✏ Manual" to enter the path directly

### Example Paths

```bash
# Home directory projects
/home/your_username/projects/my_game

# Current directory
/home/your_username/dev/bevy/my_project

# WSL accessing Windows files (slower, not recommended for development)
/mnt/c/Users/YourName/Documents/my_game
```

### System-Level Fix (Optional)

If you want to try fixing the native dialogs (may not work in all WSL setups):

```bash
# Install XDG Desktop Portal
sudo apt update
sudo apt install xdg-desktop-portal xdg-desktop-portal-gtk

# Install Zenity (fallback dialog)
sudo apt install zenity

# Note: This requires X11 or Wayland display server running
```

However, the manual entry fallback is more reliable and doesn't require system modifications.

## Files Modified

- `crates/bevy_editor_project/src/file_dialog_helper.rs` - New helper module
- `crates/bevy_editor_project/src/project_wizard.rs` - Updated to use fallback
- `crates/bevy_editor_project/src/project_manager.rs` - Updated to use fallback
- `crates/bevy_editor_project/src/lib.rs` - Added module export

## Technical Details

The implementation uses a state machine:
1. **Try native dialog first** - attempts `rfd::FileDialog::pick_folder()`
2. **Detect failure** - if dialog returns `None`, assume it failed
3. **Switch to manual mode** - shows text input with validation
4. **Validate path** - checks if path exists and is a directory
5. **Return result** - same API regardless of method used

This ensures the editor works seamlessly regardless of the environment!
