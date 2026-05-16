mod config;
mod init;
mod select;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "hatoba",
    about = "SSH ログイン時に作業ディレクトリを対話的に選択する"
)]
struct Cli {
    /// 設定ファイルのパス（省略時はデフォルト位置を使用）
    #[arg(long, global = true)]
    config: Option<PathBuf>,

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
        Command::Select => match cmd_select(cli.config) {
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
    let bin = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("hatoba"))
        .to_string_lossy()
        .into_owned();
    print!("{}", init::generate(shell, &bin));
}

fn cmd_select(config_path: Option<PathBuf>) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config = config::load(config_path)?;

    if config.dirs.is_empty() {
        return Err("no directories configured in config.toml".into());
    }

    select::run(&config)
}
