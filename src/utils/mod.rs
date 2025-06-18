use num_bigint::BigUint;

pub mod error;
pub mod location;
pub mod pretty;
pub mod theme;

/// format a BigUint number with underscores for readability
pub fn format_number(n: &BigUint) -> String {
    let s = n.to_string();
    let mut res = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            res.push('_');
        }
        res.push(c);
    }
    res.chars().rev().collect()
}
