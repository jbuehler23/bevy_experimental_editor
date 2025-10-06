/// Icon constants using Unicode characters
/// These work cross-platform without requiring external font files
pub struct Icons;

impl Icons {
    // Toolbar actions
    pub const PLAY: &'static str = "▶";
    pub const BUILD: &'static str = "🔨";
    pub const STOP: &'static str = "⏹";
    pub const SAVE: &'static str = "💾";
    pub const FOLDER_OPEN: &'static str = "📁";

    // Scene/Project actions
    pub const NEW: &'static str = "➕";
    pub const CLOSE: &'static str = "✕";
    pub const SETTINGS: &'static str = "⚙";

    // Editor tools
    pub const BRUSH: &'static str = "🖌";
    pub const ERASER: &'static str = "🧹";
    pub const BUCKET: &'static str = "🪣";
    pub const EYEDROPPER: &'static str = "💧";

    // Navigation
    pub const CHEVRON_RIGHT: &'static str = "›";
    pub const CHEVRON_DOWN: &'static str = "⌄";
    pub const ARROW_UP: &'static str = "↑";
    pub const ARROW_DOWN: &'static str = "↓";
    pub const ARROW_LEFT: &'static str = "←";
    pub const ARROW_RIGHT: &'static str = "→";

    // Scene objects
    pub const CAMERA: &'static str = "📷";
    pub const SPRITE: &'static str = "🖼";
    pub const LIGHT: &'static str = "💡";
    pub const AUDIO: &'static str = "🔊";
    pub const NODE: &'static str = "⬡";

    // Components
    pub const TRANSFORM: &'static str = "⛶";
    pub const PHYSICS: &'static str = "⚛";
    pub const SCRIPT: &'static str = "📄";
    pub const TILEMAP: &'static str = "⊞";

    // Status
    pub const SUCCESS: &'static str = "✓";
    pub const ERROR: &'static str = "✕";
    pub const WARNING: &'static str = "⚠";
    pub const INFO: &'static str = "ℹ";

    // File browser
    pub const FILE: &'static str = "📄";
    pub const FOLDER: &'static str = "📁";
    pub const FOLDER_CLOSED: &'static str = "📁";
    pub const FOLDER_OPEN_ALT: &'static str = "📂";

    // Inspector
    pub const EYE: &'static str = "👁";
    pub const EYE_CLOSED: &'static str = "🚫";
    pub const LOCK: &'static str = "🔒";
    pub const UNLOCK: &'static str = "🔓";
}

/// Helper trait for adding icons to buttons/labels
pub trait IconLabel {
    fn with_icon(self, icon: &str) -> String;
}

impl IconLabel for &str {
    fn with_icon(self, icon: &str) -> String {
        format!("{} {}", icon, self)
    }
}

impl IconLabel for String {
    fn with_icon(self, icon: &str) -> String {
        format!("{} {}", icon, self)
    }
}

/// Create a button label with an icon
pub fn icon_label(icon: &str, text: &str) -> String {
    format!("{} {}", icon, text)
}

/// Create an icon-only label (just the icon)
pub fn icon_only(icon: &str) -> &str {
    icon
}
