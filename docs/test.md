# hatoba テストガイド

テストコードを整備するガイドです。

---

## Step 1: Rust のテストの仕組み

### 基本構造

Rust のユニットテストは **テスト対象のファイルの末尾** に書きます。

```rust
#[cfg(test)]          // ← test ビルド時のみコンパイルされる
mod tests {
    use super::*;     // ← 親モジュール（テスト対象）を全インポート

    #[test]           // ← これがあると cargo test が実行してくれる関数
    fn my_test() {
        assert_eq!(1 + 1, 2);   // 等値チェック。失敗するとパニック
        assert!(true);           // 真偽チェック
    }
}
```

### よく使うマクロ

| マクロ | 用途 |
|---|---|
| `assert_eq!(a, b)` | a == b を確認。失敗時に両方の値を表示 |
| `assert_ne!(a, b)` | a != b を確認 |
| `assert!(expr)` | expr が true を確認 |
| `panic!("msg")` | テストを強制失敗させる |

### テスト実行コマンド

```bash
cargo test                      # 全テストを実行
cargo test config               # "config" を含む名前のテストのみ
cargo test -- --nocapture       # println! の出力を表示
cargo test -- --test-threads=1  # 直列実行（デバッグ時に便利）
```

---

## Step 2: config モジュール — `Dir::display()`

**最初は副作用のない純粋関数からテストを書くのが鉄則です。**

`Dir::display()` は引数なし・ファイルI/Oなし・ランダム性なしの単純な関数なので、テスト入門に最適です。

[src/config.rs](../src/config.rs) の末尾に追加します：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_returns_label_when_present() {
        let dir = Dir {
            path: "/home/user".to_string(),
            label: Some("myproject".to_string()),
            default: false,
        };
        assert_eq!(dir.display(), "myproject");
    }

    #[test]
    fn display_returns_path_when_no_label() {
        let dir = Dir {
            path: "/home/user".to_string(),
            label: None,
            default: false,
        };
        assert_eq!(dir.display(), "/home/user");
    }
}
```

---

## Step 3: config モジュール — `load()`

`load()` はファイルI/Oがあるため、**一時ファイル** を使ってテストします。

### `tempfile` クレートの追加

`Cargo.toml` に以下を追記します：

```toml
[dev-dependencies]
tempfile = "3"
```

`[dev-dependencies]` は `cargo test` 時のみ使われ、リリースビルドには含まれません。

### テストコード

`src/config.rs` の `#[cfg(test)] mod tests` 内に追加します：

```rust
use std::io::Write as IoWrite;

#[test]
fn load_returns_error_when_file_missing() {
    let result = load(Some(PathBuf::from("/nonexistent/path/config.toml")));
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("config file not found"));
    assert!(msg.contains("hint:"));
}

#[test]
fn load_parses_valid_toml() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, r#"
[[dirs]]
path = "/tmp/foo"
label = "foo"
default = true
"#).unwrap();

    let config = load(Some(file.path().to_path_buf())).unwrap();
    assert_eq!(config.dirs.len(), 1);
    assert_eq!(config.dirs[0].path, "/tmp/foo");
    assert_eq!(config.dirs[0].label, Some("foo".to_string()));
    assert!(config.dirs[0].default);
}

#[test]
fn load_expands_home_variable() {
    let home = dirs::home_dir().unwrap();
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, r#"
[[dirs]]
path = "$HOME/workspace"
"#).unwrap();

    let config = load(Some(file.path().to_path_buf())).unwrap();
    let expected = format!("{}/workspace", home.display());
    assert_eq!(config.dirs[0].path, expected);
}
```

---

## Step 4: cmd/init モジュール — `generate()`

シェルスクリプトを生成する純粋関数です。出力文字列に期待する文字列が含まれるかを確認します。

[src/cmd/init.rs](../src/cmd/init.rs) の末尾に追加します：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_bash_contains_bin_path() {
        let script = generate("bash", "/usr/local/bin/hatoba");
        assert!(script.contains("/usr/local/bin/hatoba select"));
    }

    #[test]
    fn generate_bash_checks_login_shell() {
        let script = generate("bash", "hatoba");
        assert!(script.contains(r#"[[ "$0" == "-bash" ]]"#));
    }

    #[test]
    fn generate_zsh_contains_bin_path() {
        let script = generate("zsh", "/usr/local/bin/hatoba");
        assert!(script.contains("/usr/local/bin/hatoba select"));
    }

    #[test]
    fn generate_zsh_checks_login_shell() {
        let script = generate("zsh", "hatoba");
        assert!(script.contains("-o login"));
    }

    #[test]
    fn generate_bash_and_zsh_differ() {
        let bash = generate("bash", "hatoba");
        let zsh = generate("zsh", "hatoba");
        assert_ne!(bash, zsh);
    }
}
```

---

## Step 5: cmd/select モジュール — ロジック部分のみ

`select::run()` は `dialoguer::Select` を通じて TTY（端末）に描画するため、
**テスト環境（TTY なし）では TUI 部分を呼び出せません。**

TTY を必要としない分岐（`dirs.len() == 1`）のみテスト可能です。

[src/cmd/select.rs](../src/cmd/select.rs) の末尾に追加します：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Dir};

    fn make_config(paths: &[&str]) -> Config {
        Config {
            dirs: paths
                .iter()
                .map(|p| Dir {
                    path: p.to_string(),
                    label: None,
                    default: false,
                })
                .collect(),
        }
    }

    #[test]
    fn run_returns_single_dir_without_interaction() {
        let config = make_config(&["/tmp/only"]);
        let result = run(&config).unwrap();
        assert_eq!(result, Some("/tmp/only".to_string()));
    }
}
```

> **Note:** 複数ディレクトリ時のテストは TTY モックが必要で複雑になるため、今回はスコープ外とします。

---

## Step 6: cmd/list — `format_line()` のテスト

`run()` は stdout に直接出力するため、内部の `format_line()` 関数をテストします。

```rust
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
    fn format_line_path_only_when_no_label() {
        let d = dir("/tmp/foo", None, false);
        assert_eq!(format_line(&d), "/tmp/foo");
    }

    #[test]
    fn format_line_label_and_path_when_label_present() {
        let d = dir("/tmp/foo", Some("foo"), false);
        assert_eq!(format_line(&d), "foo  /tmp/foo");
    }

    #[test]
    fn format_line_label_and_path_with_default_marker() {
        let d = dir("/tmp/foo", Some("foo"), true);
        assert_eq!(format_line(&d), "foo  /tmp/foo  (default)");
    }
}
```

---

## Step 7: cmd/add・remove・default — tempfile パターン

ファイルI/Oのあるコマンドは `tempfile::NamedTempFile` で一時設定ファイルを用意し、
実行後に `config::load()` で結果を検証します。

```rust
fn make_config_file(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, "{content}").unwrap();
    file
}

#[test]
fn add_appends_new_entry() {
    let file = make_config_file("[[dirs]]\npath = \"/tmp/existing\"\n");
    run(Some(file.path().to_path_buf()), "/tmp/new", None, false).unwrap();
    let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
    assert_eq!(config.dirs.len(), 2);
}
```

---

## 最終確認

```bash
cargo test
```

期待する出力：

```
running 26 tests
test cmd::add::tests::add_appends_new_entry ... ok
test cmd::add::tests::add_creates_file_when_missing ... ok
test cmd::add::tests::add_fails_on_duplicate_path ... ok
test cmd::add::tests::add_with_default_clears_existing_defaults ... ok
test cmd::add::tests::add_with_label ... ok
test cmd::default::tests::default_fails_when_path_not_found ... ok
test cmd::default::tests::default_hints_trailing_slash_difference ... ok
test cmd::default::tests::default_sets_target_and_clears_others ... ok
test cmd::init::tests::generate_bash_and_zsh_differ ... ok
test cmd::init::tests::generate_bash_checks_login_shell ... ok
test cmd::init::tests::generate_bash_contains_bin_path ... ok
test cmd::init::tests::generate_zsh_checks_login_shell ... ok
test cmd::init::tests::generate_zsh_contains_bin_path ... ok
test cmd::list::tests::format_line_label_and_path_when_label_present ... ok
test cmd::list::tests::format_line_label_and_path_with_default_marker ... ok
test cmd::list::tests::format_line_path_only_when_no_label ... ok
test cmd::list::tests::format_line_path_only_with_default_marker ... ok
test cmd::remove::tests::remove_deletes_entry ... ok
test cmd::remove::tests::remove_fails_when_path_not_found ... ok
test cmd::remove::tests::remove_hints_trailing_slash_difference ... ok
test cmd::select::tests::run_returns_single_dir_without_interaction ... ok
test config::tests::display_returns_label_when_present ... ok
test config::tests::display_returns_path_when_no_label ... ok
test config::tests::load_expands_home_variable ... ok
test config::tests::load_parses_valid_toml ... ok
test config::tests::load_returns_error_when_file_missing ... ok

test result: ok. 26 passed; 0 failed
```

---

## テスト設計の指針（まとめ）

| 関数の種類 | テスト方法 |
|---|---|
| 純粋関数（入力→出力） | そのまま呼び出して assert |
| ファイルI/Oあり | `tempfile::NamedTempFile` で一時ファイルを用意 |
| TTY/端末依存 | TUI を使わない分岐のみテスト。残りは手動確認 |
| stdout 出力のみ | 内部ヘルパー関数に切り出してテスト |
