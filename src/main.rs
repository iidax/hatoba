mod cmd;
mod config;
mod db;
mod messages;

use clap::{Parser, Subcommand, ValueEnum};
use config::{Config, Dir, Language};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "hatoba",
    about = "Interactively select a working directory on SSH login",
    version
)]
struct Cli {
    /// Path to config file (defaults to ~/.config/hatoba/config.toml)
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Path to DB file (defaults to ~/.local/share/hatoba/hatoba.db)
    #[arg(long, global = true)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Output shell integration snippet to stdout
    Init {
        /// Target shell
        shell: Shell,
    },
    /// Interactively select a working directory and print its path to stdout
    Select,
    /// List all registered directories
    List,
    /// Add a directory to the candidate list
    Add {
        /// Path of the directory to add
        path: String,
        /// Display name (optional)
        #[arg(long)]
        label: Option<String>,
        /// Set as the default selection
        #[arg(long)]
        default: bool,
    },
    /// Change the default selection
    Default {
        /// Path of the directory to set as default
        path: String,
    },
    /// Remove a directory from the candidate list
    Remove {
        /// Path of the directory to remove
        path: String,
    },
    /// Show or set the display language
    Lang {
        /// Language to set (en / ja). Omit to show the current setting
        language: Option<LangArg>,
    },
    /// Swap the list positions of two directories
    Swap {
        /// First directory path
        path1: String,
        /// Second directory path
        path2: String,
    },
}

#[derive(ValueEnum, Clone)]
enum Shell {
    Bash,
    Zsh,
}

#[derive(ValueEnum, Clone)]
enum LangArg {
    En,
    Ja,
}

fn main() {
    let cli = Cli::parse();

    let config = Config::load(cli.config.clone());
    let msg = messages::get(&config.language);

    match cli.command {
        Command::Init { shell } => {
            let name = match shell {
                Shell::Bash => "bash",
                Shell::Zsh => "zsh",
            };
            cmd_init(name);
        }
        Command::Select => match cmd_select(cli.db, msg) {
            Ok(Some(path)) => println!("{path}"),
            Ok(None) => process::exit(1),
            Err(e) => {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        },
        Command::List => match load_dirs(cli.db) {
            Ok(dirs) => cmd::list::run(&dirs, msg),
            Err(e) => {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        },
        Command::Add {
            path,
            label,
            default,
        } => {
            if let Err(e) = cmd::add::run(cli.db, &path, label, default, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Default { path } => {
            if let Err(e) = cmd::default::run(cli.db, &path, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Remove { path } => {
            if let Err(e) = cmd::remove::run(cli.db, &path, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Lang { language } => {
            let lang = language.map(|l| match l {
                LangArg::En => Language::En,
                LangArg::Ja => Language::Ja,
            });
            if let Err(e) = cmd::lang::run(cli.config, lang) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Swap { path1, path2 } => {
            if let Err(e) = cmd::swap::run(cli.db, &path1, &path2, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
    }
}

fn load_dirs(db_path: Option<PathBuf>) -> Result<Vec<Dir>, Box<dyn std::error::Error>> {
    db::Db::open(db_path)
        .and_then(|db| db.list_dirs())
        .or_else(|_| Ok(vec![]))
}

fn cmd_init(shell: &str) {
    let bin = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("hatoba"))
        .to_string_lossy()
        .into_owned();
    print!("{}", cmd::init::generate(shell, &bin));
}

fn cmd_select(
    db_path: Option<PathBuf>,
    msg: &messages::Msg,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let dirs = load_dirs(db_path)?;
    if dirs.is_empty() {
        return Err("no directories registered. Use 'hatoba add <path>' to add one.".into());
    }
    cmd::select::run(&dirs, msg)
}
