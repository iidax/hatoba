mod config;
mod init;
mod select;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = match args.get(1).map(String::as_str) {
        Some("init") => {
            let shell = args.get(2).map(String::as_str).unwrap_or("");
            cmd_init(shell)
        }
        Some("select") => cmd_select(),
        _ => {
            eprintln!("Usage: hatoba <init <bash|zsh> | select>");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("hatoba: {e}");
        process::exit(1);
    }
}

fn cmd_init(shell: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = init::generate(shell)?;
    print!("{script}");
    Ok(())
}

fn cmd_select() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load()?;

    if config.dirs.is_empty() {
        return Err("no directories configured in config.toml".into());
    }

    let path = if config.dirs.len() == 1 {
        config.dirs[0].path.clone()
    } else {
        match select::run(&config)? {
            Some(p) => p,
            None => process::exit(1),
        }
    };

    println!("{path}");
    Ok(())
}
