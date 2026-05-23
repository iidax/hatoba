# TOML の `[[dirs]]` を SQLite へ移行する

## 概要

`[[dirs]]` の管理を TOML から SQLite に移行する。
`[settings]`（language など）は引き続き `~/.config/hatoba/config.toml` で管理。

---

## Phase 1: `[[dirs]]` を SQLite へ移行

### 変更方針

- `~/.config/hatoba/config.toml` には `[settings]` のみ残す
- `[[dirs]]` の読み書きを `~/.local/share/hatoba/hatoba.db`（SQLite）に移行
- 既存 TOML の `[[dirs]]` は破壊的変更として削除（リリース前のため互換不要）
- DB ファイルは初回コマンド実行時に自動生成（遅延生成）

### DB スキーマ

```sql
CREATE TABLE dirs (
    id         INTEGER PRIMARY KEY,
    path       TEXT    NOT NULL UNIQUE,
    label      TEXT,
    position   INTEGER NOT NULL DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

-- is_default の排他制御（テーブル全体で TRUE は高々1件）
CREATE UNIQUE INDEX idx_dirs_single_default
    ON dirs (is_default)
    WHERE is_default = TRUE;
```

#### 設計メモ

**`id`**  
`AUTOINCREMENT` は付けない。`INTEGER PRIMARY KEY` のみで SQLite が `MAX(id) + 1` で採番する。
削除した id が再利用される可能性はあるが、hatoba では id の再利用は無害なため不要。

**`is_default`**  
`BOOLEAN` 型（Postgres 互換）。SQLite 内部では `0/1` の INTEGER で格納される。
Partial Unique Index により DB レベルで排他制御する。

**`position`（一意性の制御）**  
Postgres では以下で表現できる：
```sql
position INTEGER NOT NULL UNIQUE DEFERRABLE INITIALLY DEFERRED
```
トランザクション終了時にのみ制約を評価するため、並び替え操作中の一時的な重複を許容できる。

**SQLite は `UNIQUE` への `DEFERRABLE` を非サポート**（外部キーのみ対応）のため、
`position` に UNIQUE 制約は付けず、アプリケーション側で重複しない値を採番する。

### マイグレーションファイルの命名規則

`rusqlite_migration` の `from-directory` 機能を使用。
`include_dir` クレートでコンパイル時にバイナリへ埋め込む。

**ディレクトリ構造:**

```
migrations/
  01-initial_schema/
    up.sql          ← CREATE TABLE など
  02-add_something/
    up.sql          ← 将来の変更例（down.sql も追加可）
```

**依存追加（Cargo.toml）:**

```toml
include_dir = "0.7"
rusqlite_migration = { version = "=2.5.0", features = ["from-directory"] }
```

**Rust 側での登録:**

```rust
use include_dir::{include_dir, Dir};
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/migrations");

// Db::open() 内で:
let migrations = Migrations::from_directory(&MIGRATIONS_DIR)?;
migrations.to_latest(&mut conn)?;
```

**マイグレーション適用タイミング:** `Db::open()` の中で `MIGRATIONS.to_latest(&mut conn)?` を実行する。
すべてのコマンドが DB を使う前に自動でスキーマが最新化される。

### 作成・変更ファイル一覧

| 操作 | ファイル | 内容 |
|---|---|---|
| 新規 | `migrations/20250524_initial_schema.sql` | dirs テーブル定義 |
| 新規 | `src/db.rs` | DB open / CRUD / デフォルトパス |
| 変更 | `src/config.rs` | `dirs` フィールドを `Config` から除去、`load_settings()` を追加 |
| 変更 | `src/main.rs` | `--db` フラグ追加、DB から dirs を取得して `Config` を構築 |
| 変更 | `src/cmd/add.rs` | TOML 操作 → `Db::insert_dir()` |
| 変更 | `src/cmd/remove.rs` | TOML 操作 → `Db::remove_dir()` |
| 変更 | `src/cmd/default.rs` | TOML 操作 → `Db::set_default()` |
| 変更 | `src/cmd/list.rs` | `Config` 受け取りのまま（変更最小） |
| 変更 | `src/cmd/select.rs` | `Config` 受け取りのまま（変更最小） |
| 変更 | `samples/config_ja.toml`, `config_en.toml` | `[[dirs]]` セクション削除 |

### `src/db.rs` の公開インターフェース

```rust
pub struct Db { conn: Connection }

impl Db {
    pub fn open(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>>
    pub fn insert_dir(&mut self, path: &str, label: Option<&str>, default: bool) -> Result<()>
    pub fn remove_dir(&mut self, path: &str) -> Result<()>
    pub fn set_default(&mut self, path: &str) -> Result<()>
    pub fn list_dirs(&self) -> Result<Vec<Dir>>
    pub fn path_exists(&self, path: &str) -> Result<bool>
    pub fn all_paths(&self) -> Result<Vec<String>>  // not_found ヒント用
}

pub fn db_path_default() -> Result<PathBuf, Box<dyn std::error::Error>>
// home_dir().join(".local/share/hatoba/hatoba.db")
```

### `src/config.rs` の変更点

- `Config.dirs: Vec<Dir>` フィールドを削除
- `load()` を `load_settings(path) -> Result<Settings>` に改名
- `load_language()` はそのまま維持
- `Dir` 構造体は `db.rs` で使うため残す（`#[derive(Deserialize)]` は削除）

### `src/main.rs` の変更点

`--db` グローバルフラグを追加:

```rust
/// Path to DB file (defaults to ~/.local/share/hatoba/hatoba.db)
#[arg(long, global = true)]
db: Option<PathBuf>,
```

Select / List の dirs 取得を DB から行う:

```rust
let settings = config::load_settings(cli.config.clone()).unwrap_or_default();
let dirs = db::Db::open(cli.db.clone())
    .and_then(|db| db.list_dirs())
    .unwrap_or_default();  // DB 未作成時は空リストとして扱う
let config = Config { settings, dirs };
```

### テストについて

各コマンドは `db_path: Option<PathBuf>` を引数に取る。
テスト内では `tempfile::tempdir()` で一時ディレクトリを生成し、その中のパスを渡す。
`tempfile` は OS の一時ディレクトリ（macOS は `/private/tmp` 以下）を使用するため、
`~/.local/share/hatoba/` を汚染しない。

```rust
#[test]
fn add_creates_db_when_missing() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    run(Some(db_path.clone()), "/tmp/new", None, false, &EN).unwrap();
    let db = Db::open(Some(db_path)).unwrap();
    assert_eq!(db.list_dirs().unwrap().len(), 1);
}
```

### 検証手順

1. `cargo build` — コンパイル通過
2. `cargo test` — 全テスト通過
3. `hatoba add ~/tmp/test --label test` → `~/.local/share/hatoba/hatoba.db` の `dirs` テーブルに行が入る
4. `hatoba list` → 追加したディレクトリが表示される
5. `hatoba default ~/tmp/test` → `is_default = 1` に更新される
6. `hatoba remove ~/tmp/test` → 行が削除される
7. `hatoba lang ja` → `~/.config/hatoba/config.toml` のみ更新される（DB は変化しない）
