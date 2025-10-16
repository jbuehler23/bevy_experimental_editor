/// Icon constants using ASCII-safe characters
/// Fallback to simple ASCII to avoid rendering issues with Unicode/emoji
pub struct Icons;

impl Icons {
    // Toolbar actions
    pub const PLAY: &'static str = "�-";
    pub const BUILD: &'static str = "[B]";
    pub const STOP: &'static str = "�-�";
    pub const SAVE: &'static str = "[S]";
    pub const FOLDER_OPEN: &'static str = "[F]";

    // Scene/Project actions
    pub const NEW: &'static str = "[+]";
    pub const CLOSE: &'static str = "A-";
    pub const SETTINGS: &'static str = "[*]";

    // Editor tools
    pub const BRUSH: &'static str = "[BR]";
    pub const ERASER: &'static str = "[ER]";
    pub const BUCKET: &'static str = "[BK]";
    pub const EYEDROPPER: &'static str = "[ED]";

    // Navigation
    pub const CHEVRON_RIGHT: &'static str = ">";
    pub const CHEVRON_DOWN: &'static str = "v";
    pub const ARROW_UP: &'static str = "^";
    pub const ARROW_DOWN: &'static str = "v";
    pub const ARROW_LEFT: &'static str = "<";
    pub const ARROW_RIGHT: &'static str = ">";

    // Scene objects
    pub const CAMERA: &'static str = "[C]";
    pub const SPRITE: &'static str = "[SP]";
    pub const LIGHT: &'static str = "[L]";
    pub const AUDIO: &'static str = "[A]";
    pub const NODE: &'static str = "[N]";

    // Components
    pub const TRANSFORM: &'static str = "[T]";
    pub const PHYSICS: &'static str = "[P]";
    pub const SCRIPT: &'static str = "[SC]";
    pub const TILEMAP: &'static str = "[TM]";

    // Status
    pub const SUCCESS: &'static str = "�o\"";
    pub const ERROR: &'static str = "A-";
    pub const WARNING: &'static str = "!";
    pub const INFO: &'static str = "i";
    pub const CLIPBOARD: &'static str = "[CP]";

    // File browser
    pub const FILE: &'static str = "[ ]";
    pub const FOLDER: &'static str = "[D]";
    pub const FOLDER_CLOSED: &'static str = "[D]";
    pub const FOLDER_OPEN_ALT: &'static str = "[D]";
    pub const IMAGE: &'static str = "[IMG]";
    pub const REFRESH: &'static str = "[R]";

    // Inspector
    pub const EYE: &'static str = "[V]";
    pub const EYE_CLOSED: &'static str = "[H]";
    pub const LOCK: &'static str = "[L]";
    pub const UNLOCK: &'static str = "[U]";
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
