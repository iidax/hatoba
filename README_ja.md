# hatoba 🛳️

ターミナル起動時/SSH ログイン時に作業ディレクトリを対話的に選択する Bash/Zsh プラグイン。

## 概要

`hatoba` は SSH ログイン時にディレクトリ選択メニューを表示し、
矢印キーで作業ディレクトリを選んで `cd` するツールです。

- ✅ 矢印キー（↑↓）でディレクトリを選択
- ✅ `config.toml` で選択肢・デフォルトを管理
- ✅ SSH + インタラクティブ + ログインシェルのみ発動

## インストール

```bash
cargo install hatoba
```

## セットアップ

### 1. シェルに統合する

**zsh ユーザーの場合**

下記のコマンドを実行してください。

```
echo 'eval "$(hatoba init zsh)"' >> ~/.zshrc

```

もしくは、直接 `~/.zshrc` に以下を追加します：

```zsh
eval "$(hatoba init zsh)"
```

追記後、シェルを再起動するか `source ~/.zshrc` を実行してください。

**bash ユーザーの場合**

下記のコマンドを実行してください。

```bash
echo 'eval "$(hatoba init bash)"' >> ~/.bashrc
```

もしくは、直接 `~/.bashrc` に以下を追加します：

```bash
eval "$(hatoba init bash)"
```

追記後、シェルを再起動するか `source ~/.bashrc` を実行してください。


### 2. 確認

hatoba を利用できることを確認しましょう。

```bash
# バージョン確認
hatoba --version

# ヘルプ確認
hatoba --help

```

### 3. ディレクトリを登録する

```bash
# 候補を追加（初回はファイルも自動作成されます）
hatoba add ~/Workspace/myproject --label myproject --default

# 候補をさらに追加
hatoba add ~/Workspace/other --label other

# 登録内容を確認
hatoba list
```

候補のデフォルトを変更することもできます。

```bash
# デフォルト選択を変更
hatoba default ~/Workspace/foo
```


### 補足

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

---

## 主要コマンドの一覧

| コマンド | 説明 |
|---|---|
| `hatoba list` | 登録済みディレクトリを一覧表示 |
| `hatoba add <path> [--label <name>] [--default]` | ディレクトリを追加 |
| `hatoba remove <path>` | ディレクトリを削除 |
| `hatoba default <path>` | デフォルト選択を変更 |

