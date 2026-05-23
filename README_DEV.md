# 開発

## 前提条件

- Rust 1.85.0 以上

## 動作イメージ

```
hatoba: 作業ディレクトリを選択
  myproject  ~/Workspace/myproject  (default)
  other      ~/Workspace/other
```

## 発動条件

以下をすべて満たす場合のみメニューが表示されます：

| 条件 | 内容 |
|---|---|
| `-t 0` / `-t 1` | インタラクティブな端末セッションである |
| `login_shell` | ログインシェルである |
| `$PWD == $HOME` | カレントディレクトリがホームディレクトリである |



## インストール

```bash
git clone https://github.com/iidax/hatoba
cd hatoba
cargo build
```

```bash
cargo test          # テスト
cargo fmt           # フォーマット
cargo clippy        # 静的解析
```

リリースビルドとインストール：

```bash
cargo build --release   # 最適化バイナリを target/release/ に生成
cargo install --path .  # ~/.cargo/bin/ にインストール
```

## ライセンス

MIT
