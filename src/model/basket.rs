use super::MAX_SL_LEN;


pub fn is_valid_name(s: &str) -> bool {
    use std::ascii::AsciiExt;

    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !s.starts_with('-')
        && s.len() < MAX_SL_LEN
}
