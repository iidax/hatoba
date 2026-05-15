# hatoba 仕様書

## 概要

`hatoba` は SSH ログイン時にディレクトリ選択メニューを表示し、  
矢印キーで作業ディレクトリを選んで `cd` できる CLI ツールです。

```
=== hatoba: 作業ディレクトリを選択 ===
↑↓ で移動、Enter で確定、Esc でキャンセル

  home  /home/username
▶ ssd4  /mnt/ssd4/username       (default)
```

---

## サブコマンド

### `hatoba init <bash|zsh>`

シェルの設定ファイル（`.bashrc` / `.zshrc`）に書く統合コードを stdout に出力します。

```bash
eval "$(hatoba init bash)"   # .bashrc に追記
eval "$(hatoba init zsh)"    # .zshrc  に追記
```

出力されるシェルコードは `_hatoba_hook` 関数を定義し、  
**ログインシェル起動時** に自動で呼び出します。

#### 出力されるコード（bash）

```bash
_hatoba_hook() {
  if [[ -n "${SSH_CONNECTION}" && -t 0 && -t 1 ]]; then
    local dir
    dir=$(hatoba select) && cd "${dir}"
  fi
}
[[ "$0" == "-bash" ]] && _hatoba_hook
```

`hatoba select` の終了コードが 0（正常選択）のときだけ `cd` します。  
キャンセルした場合は `&&` が短絡評価で止まるため `cd` しません。

#### 発動条件（シェル側で判定）

| 変数 / 条件 | 意味 |
|---|---|
| `$SSH_CONNECTION` が空でない | SSH 接続である |
| `-t 0` かつ `-t 1` | 標準入出力が tty（インタラクティブ端末）である |
| `$0 == "-bash"` / zsh の `-o login` | ログインシェルである |

`scp` / `rsync` / 踏み台経由では `-t` 条件を満たさないためスキップされます。

---

### `hatoba select`

インタラクティブなディレクトリ選択 TUI を **stderr に表示** し、  
選択されたパスを **stdout に出力** して終了します。

シェルが `dir=$(hatoba select)` で stdout を変数に取り込み、`cd` に使います。  
TUI を stderr に分けるのは、`$()` が stdout だけをキャプチャするためです。

#### 挙動一覧

| 状況 | 動作 |
|---|---|
| 設定ファイルなし | stderr にエラー表示、exit 1 |
| dirs が 0 件 | stderr にエラー表示、exit 1 |
| dirs が 1 件 | メニューを出さず、そのパスを stdout に出力して exit 0 |
| dirs が 2 件以上 | TUI メニュー表示（下記参照） |
| Enter で確定 | 選択パスを stdout に出力して exit 0 |
| Esc / Ctrl+C | 何も出力せず exit 1（`cd` はしない） |

#### キー操作

| キー | 動作 |
|---|---|
| `↑` / `↓` | カーソル移動（端を超えると折り返す） |
| `Enter` | 現在の選択を確定 |
| `Esc` / `Ctrl+C` | キャンセル（現在のディレクトリに留まる） |

---

## 設定ファイル

パス: `~/.config/hatoba/config.toml`

```toml
# デフォルトで選択されるディレクトリ（省略可）
default = "/mnt/ssd4/username"

[[dirs]]
path = "/home/username"
label = "home"       # 表示名（省略するとパスをそのまま表示）

[[dirs]]
path = "/mnt/ssd4/username"
label = "ssd4"
```

- `default` に一致するエントリは `(default)` ラベルを表示し、起動時のカーソル位置になります
- `default` が省略または一致なしの場合は先頭にカーソルが置かれます

---

## コード構成

```
src/
  main.rs    コマンドライン引数の解析とサブコマンドへの振り分け
  config.rs  設定ファイル（TOML）の読み込みと構造体定義
  init.rs    hatoba init が出力するシェルスクリプト文字列の定義
  select.rs  crossterm を使った TUI メニューの描画とキー操作
```

### データの流れ

```
[SSH ログイン]
    │
    ▼
シェルが _hatoba_hook を呼ぶ
    │  SSH_CONNECTION / tty / login の確認はシェル側
    ▼
dir=$(hatoba select)      ← stdout をキャプチャ
    │
    ├─ config::load()     ~/.config/hatoba/config.toml を読む
    │
    ├─ dirs.len() == 1    → パスを stdout に出力して終了
    │
    └─ dirs.len() >= 2    → TUI メニュー（stderr に描画）
            │
            ├─ Enter      → パスを stdout に出力、exit 0
            └─ Esc / ^C   → exit 1（cd しない）
    │
    ▼
cd "${dir}"               ← exit 0 のときだけ実行
```

---

## Rust コードを読む際のポイント

### `Result<T, E>` ― 失敗するかもしれない処理

```rust
// Ok(値) か Err(エラー) のどちらかを返す型
fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;  // ? → エラーなら即 return Err
    Ok(config)
}
```

`?` 演算子は「エラーなら呼び出し元に return する」構文糖衣です。

### `Option<T>` ― 値がないかもしれない

```rust
// Some(値) か None のどちらかを持つ型
let default: Option<String> = config.default;  // TOML で省略された場合は None

// パターンマッチで取り出す
match config.default {
    Some(path) => println!("default: {}", path),
    None => println!("no default"),
}
```

### `Box<dyn std::error::Error>` ― 何でも入るエラー型

複数の種類のエラー（IO エラー、TOML パースエラーなど）を一つの型で扱うための「トレイトオブジェクト」です。  
エラーの種類を気にせずまとめて返したいときに使います。

### 所有権 ― 文字列の `String` と `&str`

```rust
let s: String = "hello".to_string();  // ヒープに確保された所有権付き文字列
let r: &str = &s;                     // s を借用した参照（コピーしない）

fn display(&self) -> &str {
    self.label.as_deref().unwrap_or(&self.path)
    // as_deref: Option<String> → Option<&str> に変換
    // unwrap_or: None なら &self.path を返す
}
```

### `queue!` / `execute!` ― crossterm のターミナル制御

```rust
// queue! はコマンドをバッファに積む（まだ出力しない）
queue!(stderr, style::Print("hello\r\n"))?;

// execute! はバッファに積んで即フラッシュする
execute!(stderr, cursor::MoveUp(3), terminal::Clear(ClearType::FromCursorDown))?;
```

Raw モード（`terminal::enable_raw_mode()`）に入ると、  
Enter キーなしでキー入力を即座に受け取れるようになります。  
必ず `disable_raw_mode()` で元に戻す必要があります。
