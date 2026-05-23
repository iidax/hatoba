# hatoba ロードマップ

OSS 公開に向けた作業リストと将来計画です。

---

## v0.1.0 — 初回公開

OSS として公開するために必要な最低限の整備。

### インフラ・メタデータ

- [x] **LICENSE ファイルを追加**
  - `LICENSE`（MIT テキスト）をリポジトリルートに配置
- [x] **Cargo.toml メタデータを整備**
  - `description`, `license`, `repository`, `keywords`, `categories` を追加
- [x] **`hatoba --version` を有効化**
  - `main.rs` の `#[command(...)]` に `version` 属性を追加
### CI/CD

- [x] **GitHub Actions を追加**
  - `cargo test` の自動実行（push / PR）
  - `cargo fmt --check` と `cargo clippy -- -D warnings`
  - macOS / Linux のマトリクスビルド（`.github/workflows/ci.yml`）
- [x] **リリース自動化**
  - タグ付け時に GitHub Release を自動作成する workflow（`.github/workflows/release.yml`）



---

## v0.2.0 — 品質向上

公開後の継続的な整備。

- [x] **英語版 README を追加**（`README.md` を英語化、または `README_ja.md` に分離）
- [x] **ソースコード内の日本語を英語化**
- [x] **CONTRIBUTING.md を作成**
  - 開発環境のセットアップ手順
  - PR / Issue の送り方
- [x] **GitHub テンプレートを追加**
  - `.github/ISSUE_TEMPLATE/01-enhancement.yml`
  - `.github/ISSUE_TEMPLATE/02-bug.yml`
  - `.github/pull_request_template.md`

### 公開

- [ ] **crates.io への publish**
  - `cargo publish` で登録

---

## Backlog — 将来的な機能拡張

優先度は未定。フィードバックや需要に応じて判断します。

| アイデア | 概要 |
|---|---|
| `hatoba edit` | `$EDITOR` で `config.toml` を直接開く |
| `hatoba import` | 既存ディレクトリ一覧（`ls ~/Workspace`など）からまとめて登録 |
| 設定プロファイル | 複数の config を切り替える（サーバごとに異なる選択肢） |
| Fish シェル対応 | `hatoba init fish` を追加 |
| 選択履歴 | 最後に選んだディレクトリをデフォルトにする |
