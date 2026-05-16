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

### 1. シェルに統合する

**.zshrc の場合：**

```zsh
eval "$(hatoba init zsh)"
```

**.bashrc の場合：**

```bash
eval "$(hatoba init bash)"
```

追記後、シェルを再起動するか `source ~/.zshrc` を実行してください。

### 2. ディレクトリを登録する

```bash
# 候補を追加（初回はファイルも自動作成されます）
hatoba add ~/Workspace/myproject --label myproject --default

# 追加の候補
hatoba add ~/Workspace/other --label other

# 登録内容を確認
hatoba list
```

`~/.config/hatoba/config.toml` を直接編集することもできます：

```toml
# ~/.config/hatoba/config.toml
[[dirs]]
path = "~/Workspace/myproject"
label = "myproject"
default = true

[[dirs]]
path = "~/Workspace/other"
label = "other"
```

## 設定の管理

| コマンド | 説明 |
|---|---|
| `hatoba list` | 登録済みディレクトリを一覧表示 |
| `hatoba add <path> [--label <name>] [--default]` | ディレクトリを追加 |
| `hatoba remove <path>` | ディレクトリを削除 |
| `hatoba default <path>` | デフォルト選択を変更 |

```bash
# 例
hatoba add ~/Workspace/foo --label foo --default
hatoba remove ~/Workspace/old
hatoba default ~/Workspace/foo
```

## 発動条件

以下をすべて満たす場合のみメニューが表示されます：

| 条件 | 内容 |
|---|---|
| `-t 0` / `-t 1` | インタラクティブな端末セッションである |
| `login_shell` | ログインシェルである |
| `$PWD == $HOME` | カレントディレクトリがホームディレクトリである |

## 動作イメージ

```
hatoba: 作業ディレクトリを選択
  myproject  ~/Workspace/myproject  (default)
  other      ~/Workspace/other
```

## 開発

```bash
git clone https://github.com/iidax/hatoba
cd hatoba
cargo build
```

```bash
cargo test          # テスト
cargo fmt           # フォーマット
cargo clippy        # 静的解析
cargo build --release
cargo install --path .
```

## ライセンス

MIT
