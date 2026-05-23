# リポジトリ設定・ブランチ保護ルール

## 現状確認

### gh コマンドの確認

対象リポジトリに合わせて変数 `GH_REPO` を設定してください

```shell
export GH_REPO="iidax/hatoba"
```

```bash
# トピックを確認
gh api repos/$GH_REPO/topics --method GET

```

### ブランチ保護の確認

```bash
gh api repos/$GH_REPO/branches/main/protection
```

`Branch not protected` が返る場合は未設定。

### マージ戦略の確認

```bash
gh repo view $GH_REPO --json squashMergeAllowed,mergeCommitAllowed,rebaseMergeAllowed,deleteBranchOnMerge
```

実行結果例：

```json
{
  "deleteBranchOnMerge": false,
  "mergeCommitAllowed": true,
  "rebaseMergeAllowed": true,
  "squashMergeAllowed": true
}
```

現在の設定：

| 項目 | 現状 | 推奨 |
|------|------|------|
| squash merge | 有効 | 有効 |
| merge commit | 有効 | 無効（コミット履歴を綺麗に保つ） |
| rebase merge | 有効 | 無効（squash に統一） |
| マージ後にブランチ削除 | 無効 | 有効 |

---

## 設定手順

### 1. マージ戦略を squash のみに統一

```bash
gh api --method PATCH repos/$GH_REPO \
  --field squash_merge_commit_title=PR_TITLE \
  --field squash_merge_commit_message=PR_BODY \
  --field allow_squash_merge=true \
  --field allow_merge_commit=false \
  --field allow_rebase_merge=false \
  --field delete_branch_on_merge=true
```

### 2. main ブランチ保護ルールの設定

```bash
gh api --method PUT repos/$GH_REPO/branches/main/protection --input - <<'EOF'
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["test"]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true
  },
  "restrictions": null
}
EOF

```

設定内容：

| 項目 | 内容 |
|------|------|
| PR 必須 | 直接 push 禁止、PR 経由のみ |
| レビュー必須 | 承認 1 名以上 |
| CI 通過必須 | `test` ジョブが green であること |
| stale レビュー破棄 | 新たな push でレビュー承認を無効化 |
| strict | PR を常に最新の main ベースに保つ |

> `enforce_admins=false` にするとオーナー（iidax）は保護ルールをバイパスできます。
> 厳格にする場合は `true` に変更してください。

### 3. 設定の確認

```bash
gh api repos/$GH_REPO/branches/main/protection | \
  jq '{required_pr: .required_pull_request_reviews.required_approving_review_count, dismiss_stale_reviews: .required_pull_request_reviews.dismiss_stale_reviews, ci_contexts: .required_status_checks.contexts, enforce_admins: .enforce_admins.enabled}'
```

期待する実行結果例：

```json
{
  "required_pr": 1,                   // 必要なレビュー数
  "dismiss_stale_reviews": true,      // stale レビュー破棄
  "ci_contexts": [
    "test"                            // 必須の CI ジョブ名
  ],
  "enforce_admins": false             // 管理者によるルールバイパス
}
```

### 4. ブランチ保護の解除（必要な場合）

```bash
gh api --method DELETE repos/$GH_REPO/branches/main/protection
```

---

## 補足

- CI ジョブ名（`test`）は [.github/workflows/ci.yml](../.github/workflows/ci.yml) の `jobs:` キー名と一致させてください。
- コントリビューターは fork してから PR を送る一般的な OSS フローになります。


---

