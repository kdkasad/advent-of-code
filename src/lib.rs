use std::fmt::Display;

pub trait Solution<'a> {
    type Output: Display + 'a;
    fn part1(input: &'a str) -> Self::Output;
    fn part2(input: &'a str) -> Self::Output;
}

// Split a string into exactly `N` parts.
//
// # Panics
//
// Panics if the string cannoy be split into exactly `N` parts.
pub fn splitn<const N: usize>(s: &str, delim: char) -> [&str; N] {
    let mut parts = s.split(delim);
    let mut arr = [""; N];
    for (i, elem) in arr.iter_mut().enumerate() {
        *elem = parts.next().unwrap_or_else(|| panic!("expected {N} parts, got {i}"));
    }
    if parts.next().is_some() {
        panic!("expected {N} parts, got more");
    }
    arr
}
