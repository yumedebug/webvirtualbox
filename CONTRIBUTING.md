# WebVirtualBoxへの貢献ガイド

WebVirtualBoxプロジェクトへの貢献に興味を持っていただき、ありがとうございます！

## 🎯 貢献の種類

以下の種類の貢献を歓迎します：

- 🐛 **バグ報告**: 問題を見つけた場合
- 💡 **機能提案**: 新しい機能のアイデア
- 📝 **ドキュメント改善**: README、コメント、ガイドの改善
- 🔧 **コード貢献**: バグ修正、新機能実装
- 🧪 **テスト追加**: テストカバレッジの向上
- 🌍 **翻訳**: 多言語対応

## 🚀 開発フロー

### 1. 問題を見つける・作成する

- 既存の[Issues](https://github.com/yourusername/webvirtualbox/issues)を確認
- 新しい問題の場合は、Issueを作成して問題を説明

### 2. フォークとクローン

```bash
# リポジトリをフォーク
# GitHub上で"Fork"ボタンをクリック

# フォークしたリポジトリをクローン
git clone https://github.com/YOUR_USERNAME/webvirtualbox.git
cd webvirtualbox

# upstreamリモートを追加
git remote add upstream https://github.com/yourusername/webvirtualbox.git
```

### 3. ブランチ作成

```bash
# 最新の状態に更新
git checkout main
git pull upstream main

# 作業用ブランチを作成
git checkout -b feature/your-feature-name
# または
git checkout -b fix/issue-description
```

### 4. 開発

```bash
# 依存関係をインストール
npm install

# 開発サーバーを起動
npm run dev

# コードを変更...

# ビルドテスト
npm run build
```

### 5. コミット

```bash
# 変更をステージ
git add .

# コミット（分かりやすいメッセージで）
git commit -m "feat: add new feature description"
# または
git commit -m "fix: resolve issue with description"
```

#### コミットメッセージ規約

- `feat:` 新機能
- `fix:` バグ修正
- `docs:` ドキュメント変更
- `style:` コードスタイル（フォーマット、セミコロン等）
- `refactor:` リファクタリング
- `test:` テスト追加・修正
- `chore:` ビルドプロセス、ツール変更

### 6. プッシュとPull Request

```bash
# フォークにプッシュ
git push origin feature/your-feature-name

# GitHub上でPull Requestを作成
```

## 📋 Pull Requestガイドライン

### PR作成前に

- [ ] 最新の`main`ブランチから作成している
- [ ] `npm run build`が成功することを確認
- [ ] 既存のテストが通ることを確認
- [ ] 新しい機能にはテストを追加
- [ ] コードが既存のスタイルに従っている

### PR説明に含めること

- 変更の概要
- 関連するIssue番号（`Fixes #123`）
- テスト方法
- スクリーンショット（UI変更の場合）

## 🎨 コードスタイル

### TypeScript/React

- ESLintとPrettierの設定に従う
- 関数コンポーネントを使用
- 型定義を明示的に記述
- コメントは英語または日本語で

### Rust

- `cargo fmt`でフォーマット
- `cargo clippy`の警告を修正
- ドキュメントコメントを記述

### 命名規則

- コンポーネント: PascalCase (`VMConsole`)
- 関数・変数: camelCase (`handleStart`)
- 定数: UPPER_SNAKE_CASE (`MAX_MEMORY_SIZE`)
- ファイル名: ケバブケース (`vm-console.tsx`)

## 🧪 テスト

### テストの追加

新しい機能やバグ修正には、必ずテストを追加してください。

```bash
# テスト実行
npm test

# カバレッジ確認
npm run test:coverage
```

### テストの種類

- **ユニットテスト**: 個別の関数・コンポーネント
- **統合テスト**: 複数のコンポーネントの連携
- **E2Eテスト**: 実際のユーザーフロー

## 📚 ドキュメント

### README更新

- 新機能追加時はREADMEも更新
- 設定変更時は使い方を更新
- スクリーンショットは`docs/`ディレクトリに保存

### コードコメント

- 複雑なロジックにはコメントを追加
- 公共のAPIにはJSDoc/TSDocコメント
- Rustコードにはドキュメントコメント

## 🐛 バグ報告

バグ報告は[Issues](https://github.com/yourusername/webvirtualbox/issues)に作成してください。

### 含める情報

- 環境（OS、ブラウザ、バージョン）
- 再現手順
- 期待される動作
- 実際の動作
- エラーメッセージ（あれば）
- スクリーンショット（あれば）

## 💡 機能提案

新しい機能の提案も歓迎します！

### 提案前に

- 既存のIssuesで類似の提案がないか確認
- プロジェクトの方向性に合致しているか検討
- 実装の複雑さを考慮

### 提案方法

1. Issueを作成（`enhancement`ラベル）
2. 機能の概要を説明
3. 使用例やメリットを記載
4. 実装案があれば記載

## 🤔 質問

質問がある場合は：

- [Discussions](https://github.com/yourusername/webvirtualbox/discussions)で質問
- 既存のIssuesを検索
- READMEとドキュメントを確認

## 📝 ライセンス

貢献したコードはMITライセンスの下で公開されます。

## 🎉 感謝

すべての貢献に感謝します！あなたの助けがWebVirtualBoxをより良くします。

---

**質問がありますか？** [Discussions](https://github.com/yourusername/webvirtualbox/discussions)でお気軽にどうぞ！
