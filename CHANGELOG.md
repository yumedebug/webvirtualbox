# Changelog

このプロジェクトのすべての重要な変更はこのファイルに記録されます。

形式は [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいており、
このプロジェクトは [Semantic Versioning](https://semver.org/lang/ja/) に準拠しています。

## [Unreleased]

### Added
- 初期リリース準備
- GitHub Actions CI/CDパイプライン
- GitHub Pages自動デプロイ
- Issue/PRテンプレート
- セキュリティポリシー
- 行動規範

## [1.0.0] - 2024-XX-XX

### Added
- **VM管理機能**
  - 仮想マシンの作成・削除・起動・停止・リセット
  - VM設定のカスタマイズ（CPU、メモリ、ディスクサイズ）
  - VM一覧表示と詳細表示
  - IndexedDBによるVM設定の永続化

- **CPUエミュレーション**
  - x86 Real Mode命令セットの実装
  - レジスタ（EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP）
  - セグメントレジスタ（CS, DS, ES, SS, FS, GS）
  - フラグレジスタ（CF, PF, AF, ZF, SF, TF, IF, DF, OF）
  - 命令フェッチ・デコード・実行サイクル

- **対応命令**
  - データ転送: MOV, PUSH, POP, XCHG, LEA, LES, LDS
  - 演算: ADD, SUB, INC, DEC, CMP, XOR, AND, OR, TEST
  - 分岐: JMP, Jcc, CALL, RET
  - 文字列: MOVSB, MOVSW, STOSB, STOSW, REP
  - I/O: IN, OUT
  - 割り込み: INT, IRET, CLI, STI
  - その他: NOP, HLT, CBW, CWD, MUL, DIV, LOOP, SETcc

- **仮想デバイス**
  - VGA テキストモード（80x25）
  - PIC (8259A) 割り込みコントローラ
  - PIT (8254) プログラマブルインターバルタイマ
  - キーボードコントローラ（PS/2）

- **BIOS実装**
  - INT 10h: ビデオサービス
  - INT 13h: ディスクサービス
  - INT 16h: キーボードサービス
  - INT 19h: ブートストラップローダー
  - INT 12h: メモリサイズ取得
  - INT 15h: システムサービス

- **ストレージ**
  - 仮想ハードディスク（RAW形式）
  - 仮想CD-ROM（ISOイメージ）
  - ブートセクタ読み込み

- **表示**
  - 640x480 VGA framebuffer
  - Canvasへのリアルタイム描画
  - テキストモード文字レンダリング
  - 16色パレット

- **入力**
  - キーボード入力（Set 1 scancode）
  - マウス入力（座標変換、ボタン状態）

- **UI**
  - ダークテーマ
  - レスポンシブデザイン
  - VM作成ダイアログ
  - VM設定ダイアログ
  - VMコンソール画面
  - CPU状態表示

- **Rust VM Core**
  - CPUエミュレーション（registers, decoder, executor, flags）
  - メモリ管理
  - デバイス管理（VGA, Keyboard, PIC, PIT）
  - ストレージ管理
  - ディスプレイ管理
  - 入力管理
  - 仮想マシン統合

### Technical Details
- TypeScript + React + Vite + Tailwind CSS
- Rust + WebAssembly（オプション）
- IndexedDBによるデータ永続化
- Canvas APIによるVGA描画
- Keyboard Events / Pointer Events
- Web Workers（設計上）

[Unreleased]: https://github.com/yourusername/webvirtualbox/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/yourusername/webvirtualbox/releases/tag/v1.0.0
