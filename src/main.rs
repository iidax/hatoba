mod cmd;
mod config;

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
    /// 登録済みディレクトリを一覧表示する
    List,
    /// ディレクトリを候補に追加する
    Add {
        /// 追加するディレクトリのパス
        path: String,
        /// 表示名（省略可）
        #[arg(long)]
        label: Option<String>,
        /// デフォルト選択にする
        #[arg(long)]
        default: bool,
    },
    /// デフォルト選択を変更する
    Default {
        /// デフォルトにするディレクトリのパス
        path: String,
    },
    /// ディレクトリを候補から削除する
    Remove {
        /// 削除するディレクトリのパス
        path: String,
    },
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
        Command::List => match config::load(cli.config) {
            Ok(config) => cmd::list::run(&config),
            Err(e) => {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        },
        Command::Add { path, label, default } => {
            if let Err(e) = cmd::add::run(cli.config, &path, label, default) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Default { path } => {
            if let Err(e) = cmd::default::run(cli.config, &path) {
                eprintln!("hatoba: {e}");
                process::exit(1);
            }
        }
        Command::Remove { path } => {
            if let Err(e) = cmd::remove::run(cli.config, &path) {
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

fn cmd_select(config_path: Option<PathBuf>) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config = config::load(config_path)?;

    if config.dirs.is_empty() {
        return Err("no directories configured in config.toml".into());
    }

    cmd::select::run(&config)
}
