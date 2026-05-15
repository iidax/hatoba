mod config;
mod init;
mod select;

use clap::{Parser, Subcommand, ValueEnum};
use std::process;

#[derive(Parser)]
#[command(name = "hatoba", about = "SSH ログイン時に作業ディレクトリを対話的に選択する")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// シェル統合スニペットを stdout に出力する
    Init {
        /// 対象シェル
        shell: Shell,
    },
    /// 作業ディレクトリを対話的に選択して stdout に出力する
    Select,
}

#[derive(ValueEnum, Clone)]
enum Shell {
    Bash,
    Zsh,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { shell } => {
            let name = match shell {
                Shell::Bash => "bash",
                Shell::Zsh => "zsh",
            };
            cmd_init(name);
        }
        Command::Select => match cmd_select() {
            Ok(Some(path)) => println!("{path}"),
            Ok(None) => process::exit(1),
            Err(e) => {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        },
    }
}

fn cmd_init(shell: &str) {
    print!("{}", init::generate(shell));
}

fn cmd_select() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config = config::load()?;

    if config.dirs.is_empty() {
        return Err("no directories configured in config.toml".into());
    }

    select::run(&config)
}
