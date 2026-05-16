use crate::config::{Config, Dir};

pub fn run(config: &Config) {
    for dir in &config.dirs {
        println!("{}", format_line(dir));
    }
}

fn format_line(dir: &Dir) -> String {
    if dir.default {
        format!("{}  (default)", dir.display())
    } else {
        dir.display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(path: &str, label: Option<&str>, default: bool) -> Dir {
        Dir {
            path: path.to_string(),
            label: label.map(str::to_string),
            default,
        }
    }

    #[test]
    fn format_line_shows_default_marker() {
        let d = dir("/tmp/foo", None, true);
        assert_eq!(format_line(&d), "/tmp/foo  (default)");
    }

    #[test]
    fn format_line_no_marker_when_not_default() {
        let d = dir("/tmp/foo", None, false);
        assert_eq!(format_line(&d), "/tmp/foo");
    }

    #[test]
    fn format_line_uses_label_over_path() {
        let d = dir("/tmp/foo", Some("foo"), true);
        assert_eq!(format_line(&d), "foo  (default)");
    }
}
