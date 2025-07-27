use colored::Colorize as _;
use similar::{ChangeTag, TextDiff};

/// print_diff compares two strings and prints the differences in a unified format.
/// and returns true if there are any differences.
pub fn print_diff(expected: &str, actual: &str) -> bool {
    let diff = TextDiff::from_lines(expected, actual);
    let mut has_diff = false;
    let mut old_line = 1;
    let mut new_line = 1;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                has_diff = true;
                print!("{old_line:>4}      - {}", change.to_string().red());
                old_line += 1;
            }
            ChangeTag::Insert => {
                has_diff = true;
                print!("     {new_line:>4} + {}", change.to_string().green());
                new_line += 1;
            }
            ChangeTag::Equal => {
                print!("{old_line:>4} {new_line:>4} | {change}");
                old_line += 1;
                new_line += 1;
            }
        }
    }

    has_diff
}
