pub mod colors {
    /// ANSI color codes for text and background styling.
    pub const RESET: &str = "\x1b[0m";

    // Text colors
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    // Background colors
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";

    // Text styles
    pub const BOLD: &str = "\x1b[1m";
    pub const UNDERLINE: &str = "\x1b[4m";

    /// Wrap a string with a color and reset the style afterward.
    pub fn colorize(text: &str, color: &str) -> String {
        format!("{}{}{}", color, text, RESET)
    }
}
