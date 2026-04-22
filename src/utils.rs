use std::path::Path;

pub fn parse_int(s: &str) -> i64 {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(0)
}

pub fn safe_filename(name: &str) -> String {
    Path::new(name)
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or(name)
        .to_string()
}

pub fn clean_text(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::{parse_int, safe_filename};

    #[test]
    fn parse_int_handles_commas() {
        assert_eq!(parse_int("1,234"), 1_234);
        assert_eq!(parse_int("n/a"), 0);
    }

    #[test]
    fn safe_filename_strips_path_parts() {
        assert_eq!(safe_filename("../../etc/passwd"), "passwd");
    }
}
