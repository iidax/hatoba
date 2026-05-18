# リリース手順

## 手順

### 1. バージョンを更新する

`Cargo.toml` の `version` を更新します：

```toml
version = "0.2.0"
```

### 2. テストを通す

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

### 3. コミットする

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to v0.2.0"
```

### 4. タグを打って push する

注釈付きタグ（annotated tag）を使います。作成者・日時・メッセージが記録され、`git describe` や GitHub Releases との連携が正確になります。

```bash
git tag -a v0.2.0 -m "v0.2.0"
```

| オプション | 意味 |
|---|---|
| `-a` | annotated タグを作成する |
| `-m "..."` | タグのメッセージ（省略すると $EDITOR が開く） |

タグの内容を確認するには：

```bash
git show v0.2.0
```

push します：

```bash
git push origin main
git push origin v0.2.0
```

間違えた場合の取り消し（push 前）：

```bash
git tag -d v0.2.0
```

タグの push をトリガーに GitHub Actions が自動で以下を実行します：

- Linux (x86_64 / arm64)・macOS (arm64) 向けバイナリをビルド
- GitHub Release を作成（リリースノート自動生成）
- バイナリを Release に添付

### 5. GitHub Release を確認する

Actions の完了後、GitHub の Releases ページで内容を確認します。

必要であれば `gh release edit v0.2.0 --notes "..."` で説明文を編集できます。

---

## crates.io への publish（初回のみ）

```bash
cargo login          # API トークンを登録（初回のみ）
cargo publish --dry-run   # 事前チェック
cargo publish
```
