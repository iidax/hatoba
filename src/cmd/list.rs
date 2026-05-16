use crate::config::Config;

pub fn run(config: &Config) {
    for dir in &config.dirs {
        let marker = if dir.default { "  (default)" } else { "" };
        println!("{}{}", dir.display(), marker);
    }
}
