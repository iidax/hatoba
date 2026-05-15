# hatoba 🛳️

SSH ログイン時に作業ディレクトリを対話的に選択する Bash/Zsh プラグイン。

## 概要

`hatoba` は SSH ログイン時にディレクトリ選択メニューを表示し、
矢印キーで作業ディレクトリを選んで `cd` するツールです。

- ✅ 矢印キー（↑↓）でディレクトリを選択
- ✅ `config.toml` で選択肢・デフォルトを管理
- ✅ SSH + インタラクティブ + ログインシェルのみ発動
- ✅ `scp` / `rsync` / 踏み台経由ではスキップ

## インストール

```bash
cargo install hatoba
```

## セットアップ

### 1. 設定ファイルを作成

```toml
# ~/.config/hatoba/config.toml

default = "/mnt/ssd4/username"

[[dirs]]
path = "/home/username"
label = "home"

[[dirs]]
path = "/mnt/ssd4/username"
label = "ssd4"
```

### 2. `.bashrc` に追記

```bash
# for bash
eval "$(hatoba init bash)"

# for zsh
eval "$(hatoba init zsh)"
```

## 発動条件

以下をすべて満たす場合のみメニューが表示されます：

| 条件 | 内容 |
|---|---|
| `$SSH_CONNECTION` | SSH 接続である |
| `-t 0` / `-t 1` | インタラクティブな端末セッションである |
| `login_shell` | ログインシェルである |

## 動作イメージ


```
=== Select working directory ===
↑↓ で移動、Enter で決定

  /home/username
▶ /mnt/ssd4/username       (default)
```

## 開発

```bash
git clone https://github.com/suenaga-hiroshi/hatoba-rs
cd hatoba-rs
cargo build
```

## ライセンス

MIT
