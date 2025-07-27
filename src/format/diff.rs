use colored::Colorize as _;
use similar::{ChangeTag, TextDiff};

/// make diff
pub fn diff(original: &str, formatted: &str) -> bool {
    let diff = TextDiff::from_lines(original, formatted);
    let mut has_diff = false;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                print!("{}", format!("-{change}").red());
                has_diff = true;
            }
            ChangeTag::Insert => {
                print!("{}", format!("+{change}").green());
                has_diff = true;
            }
            ChangeTag::Equal => {
                print!(" {change}");
            }
        }
    }
    has_diff
}
