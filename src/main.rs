mod cmd;
mod config;
mod messages;

use clap::{Parser, Subcommand, ValueEnum};
use config::Language;
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

    let language = config::load_language(cli.config.clone());
    let msg = messages::get(&language);

    match cli.command {
        Command::Init { shell } => {
            let name = match shell {
                Shell::Bash => "bash",
                Shell::Zsh => "zsh",
            };
            cmd_init(name);
        }
        Command::Select => match cmd_select(cli.config, msg) {
            Ok(Some(path)) => println!("{path}"),
            Ok(None) => process::exit(1),
            Err(e) => {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        },
        Command::List => match config::load(cli.config) {
            Ok(config) => cmd::list::run(&config, msg),
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
            if let Err(e) = cmd::add::run(cli.config, &path, label, default, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Default { path } => {
            if let Err(e) = cmd::default::run(cli.config, &path, msg) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Remove { path } => {
            if let Err(e) = cmd::remove::run(cli.config, &path, msg) {
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
    }
}

fn cmd_init(shell: &str) {
    let bin = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("hatoba"))
        .to_string_lossy()
        .into_owned();
    print!("{}", cmd::init::generate(shell, &bin));
}

fn cmd_select(
    config_path: Option<PathBuf>,
    msg: &messages::Msg,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config = config::load(config_path)?;

    if config.dirs.is_empty() {
        return Err("no directories configured in config.toml".into());
    }

    cmd::select::run(&config, msg)
}
