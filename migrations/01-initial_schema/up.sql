CREATE TABLE dirs (
    id         INTEGER     PRIMARY KEY,
    path       TEXT        NOT NULL UNIQUE,
    label      TEXT,
    position   INTEGER     NOT NULL DEFAULT 0,
    is_default BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- updated_at の自動更新
-- NOTE: Postgres では DEFAULT NOW() + トリガーまたは拡張で対応する
--       SQLite は ON UPDATE CURRENT_TIMESTAMP 非対応のため AFTER UPDATE トリガーで代替する
CREATE TRIGGER dirs_updated_at
    AFTER UPDATE ON dirs
    FOR EACH ROW
BEGIN
    UPDATE dirs
    SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
    WHERE id = OLD.id;
END;

-- 全テーブル中で is_default = TRUE の行を1件に制限する
-- NOTE: Postgres では UNIQUE DEFERRABLE INITIALLY DEFERRED で表現できるが、
--       SQLite は UNIQUE への DEFERRABLE 非対応のため、Partial Index で代替する
CREATE UNIQUE INDEX idx_dirs_single_default
    ON dirs (is_default)
    WHERE is_default = TRUE;
