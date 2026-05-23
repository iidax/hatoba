use crate::config::{Config, Dir};
use crate::messages::Msg;

pub fn run(config: &Config, msg: &Msg) {
    for dir in &config.dirs {
        println!("{}", format_line(dir, msg));
    }
}

fn format_line(dir: &Dir, msg: &Msg) -> String {
    let default_marker = if dir.default { msg.default_marker } else { "" };
    match &dir.label {
        Some(label) => format!("{}  {}{}", label, dir.path, default_marker),
        None => format!("{}{}", dir.path, default_marker),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages;

    fn dir(path: &str, label: Option<&str>, default: bool) -> Dir {
        Dir {
            path: path.to_string(),
            label: label.map(str::to_string),
            default,
        }
    }

    #[test]
    fn format_line_path_only_when_no_label() {
        let d = dir("/tmp/foo", None, false);
        assert_eq!(format_line(&d, &messages::EN), "/tmp/foo");
    }

    #[test]
    fn format_line_path_only_with_default_marker() {
        let d = dir("/tmp/foo", None, true);
        assert_eq!(format_line(&d, &messages::EN), "/tmp/foo  (default)");
    }

    #[test]
    fn format_line_label_and_path_when_label_present() {
        let d = dir("/tmp/foo", Some("foo"), false);
        assert_eq!(format_line(&d, &messages::EN), "foo  /tmp/foo");
    }

    #[test]
    fn format_line_label_and_path_with_default_marker() {
        let d = dir("/tmp/foo", Some("foo"), true);
        assert_eq!(format_line(&d, &messages::EN), "foo  /tmp/foo  (default)");
    }

    #[test]
    fn format_line_japanese_default_marker() {
        let d = dir("/tmp/foo", None, true);
        assert_eq!(format_line(&d, &messages::JA), "/tmp/foo  (デフォルト)");
    }
}
