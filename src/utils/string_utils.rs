use regex::Regex;
use std::sync::LazyLock;

// Even in a single-threaded app, this is the standard way
static LINE_SPLIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\r\n|\n|\r").unwrap());

pub trait StrExtensionsUtils {
    fn split_lines(&self) -> impl Iterator<Item = &str>;
}

impl StrExtensionsUtils for &str {
    fn split_lines(&self) -> impl Iterator<Item = &str> {
        LINE_SPLIT_REGEX.split(self).filter(|s| !s.is_empty())
    }
}
