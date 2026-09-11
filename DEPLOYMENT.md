# デプロイメントガイド

このドキュメントでは、WebVirtualBoxをGitHub Pagesにデプロイする方法を説明します。

## 🚀 自動デプロイ（推奨）

GitHub Actionsを使用して、自動的にGitHub Pagesにデプロイされます。

### 設定手順

1. **GitHubリポジトリの作成**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git branch -M main
   git remote add origin https://github.com/YOUR_USERNAME/webvirtualbox.git
   git push -u origin main
   ```

2. **GitHub Pagesの有効化**
   - リポジトリの **Settings** > **Pages** を開く
   - **Source** を **GitHub Actions** に設定
   - 保存

3. **デプロイ**
   - `main`ブランチにプッシュすると自動的にデプロイされます
   - デプロイ状況は **Actions** タブで確認できます

### 設定のカスタマイズ

リポジトリ名が`webvirtualbox`以外の場合、`vite.config.js`の`base`パスを変更してください：

```javascript
const base = process.env.NODE_ENV === 'production' ? '/YOUR_REPO_NAME/' : '/';
```

## 🔧 手動デプロイ

GitHub Actionsを使用せずに手動でデプロイする場合：

### 方法1: gh-pagesブランチを使用

```bash
# ビルド
npm run build

# gh-pagesブランチにdist/をプッシュ
git subtree push --prefix dist origin gh-pages
```

### 方法2: deploy.shスクリプトを使用

```bash
#!/bin/bash
set -e

# ビルド
npm run build

# dist/に移動
cd dist

# Gitリポジトリを初期化
git init
git add -A
git commit -m 'deploy'

# gh-pagesブランチにプッシュ
git push -f https://github.com/YOUR_USERNAME/webvirtualbox.git master:gh-pages

# クリーンアップ
cd ..
rm -rf dist/.git
```

## 📝 デプロイ前の確認事項

### 1. ビルドテスト

```bash
npm run build
```

エラーがないことを確認してください。

### 2. 設定の確認

- `vite.config.js`の`base`パスが正しいか
- GitHub Pagesの設定が正しいか
- リポジトリが公開されているか（プライベートリポジトリの場合はGitHub Proが必要）

### 3. ブラウザテスト

```bash
# ローカルでビルド成果物をテスト
npm run build
npx serve dist
```

## 🌐 カスタムドメインの使用

### 1. DNS設定

ドメインレジストラで以下のDNSレコードを設定：

- **A レコード**: `185.199.108.153`
- **CNAME レコード**: `YOUR_USERNAME.github.io`

### 2. GitHub設定

- リポジトリの **Settings** > **Pages** を開く
- **Custom domain** にドメイン名を入力
- **Enforce HTTPS** を有効化

### 3. CNAMEファイルの追加

`public/CNAME`ファイルを作成：

```
yourdomain.com
```

## 🔄 デプロイのトラブルシューティング

### ビルドエラー

```bash
# 依存関係を再インストール
rm -rf node_modules package-lock.json
npm install

# キャッシュをクリア
npm cache clean --force

# 再ビルド
npm run build
```

### デプロイエラー

1. **GitHub Actionsのログを確認**
   - リポジトリの **Actions** タブを開く
   - 失敗したワークフローをクリック
   - ログを確認

2. **権限の確認**
   - リポジトリの **Settings** > **Actions** > **General**
   - **Workflow permissions** が **Read and write permissions** に設定されているか確認

3. **Pages設定の確認**
   - **Settings** > **Pages**
   - Sourceが正しく設定されているか確認

### 404エラー

- `vite.config.js`の`base`パスがリポジトリ名と一致しているか確認
- ビルドが成功しているか確認
- GitHub Pagesのデプロイが完了しているか確認

## 📊 デプロイ後の確認

1. **サイトのアクセス**
   - `https://YOUR_USERNAME.github.io/webvirtualbox/` にアクセス
   - ページが正しく表示されるか確認

2. **機能テスト**
   - VMの作成
   - VMの起動
   - 画面表示
   - キーボード入力

3. **パフォーマンス**
   - 読み込み速度
   - 実行速度
   - メモリ使用量

## 🎯 最適化のヒント

### ビルドサイズの削減

```bash
# アナライザーでバンドルサイズを確認
npm install -D rollup-plugin-visualizer
```

### キャッシュ戦略

GitHub Pagesは自動的にキャッシュを設定します。必要に応じて：

```javascript
// vite.config.js
export default defineConfig({
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
        },
      },
    },
  },
});
```

### 画像の最適化

- スクリーンショットは圧縮する
- 可能な限りSVGを使用
- WebP形式を検討

## 📚 参考リンク

- [GitHub Pages ドキュメント](https://docs.github.com/ja/pages)
- [Vite デプロイガイド](https://ja.vitejs.dev/guide/static-deploy.html)
- [GitHub Actions ドキュメント](https://docs.github.com/ja/actions)

---

**質問がありますか？** [Issues](https://github.com/yourusername/webvirtualbox/issues)でお気軽にどうぞ！
