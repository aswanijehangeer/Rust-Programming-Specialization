/// Retruns a string wrapped in ANSI red color codes.
/// # Examples:
/// ```
/// use cli_utils::colors::red;
/// let red_string = red("This is a red string");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
