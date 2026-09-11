# WebVirtualBox

ブラウザ上で動作する軽量な仮想マシンソフトウェアです。
VirtualBoxのWeb版として、ブラウザだけで仮想マシンを作成・起動・操作できます。

## 概要

WebVirtualBoxは、ブラウザ上でISOイメージからゲストOSを実際にブートできる仮想マシン環境です。
x86 CPUエミュレーション、仮想メモリ、仮想ディスク、VGA表示、キーボード/マウス入力をサポートしています。

## アーキテクチャ

```
Browser
│
├── React UI
│   ├── VM Manager (VM一覧・管理)
│   ├── VM Creation (新規作成ダイアログ)
│   ├── VM Settings (設定変更)
│   ├── VM Console (実行画面・Canvas)
│   └── Storage Manager (IndexedDB)
│
├── TypeScript VM Core
│   ├── CPU Emulator (x86命令実行)
│   ├── Memory Manager (仮想RAM)
│   ├── Device Manager (PIC, PIT, VGA, Keyboard)
│   ├── Storage (Disk, ISO)
│   └── Display (Framebuffer → Canvas)
│
└── Rust VM Core (vm-core/)
    ├── CPU (registers, decoder, executor, flags)
    ├── Memory
    ├── Devices (VGA, Keyboard, PIC, PIT)
    ├── Storage
    ├── Display
    ├── Input
    └── Machine (統合)
```

## 技術スタック

### Frontend
- **TypeScript** - メインアプリケーション言語
- **React** - UIフレームワーク
- **Vite** - ビルドツール
- **Tailwind CSS** - スタイリング

### VM Runtime
- **Rust** (vm-core/) - CPUエミュレーション、デバイス実装
- **WebAssembly** - Rustコードのブラウザ実行用
- **TypeScript VM Core** - フォールバック実装（WASM未使用時）

### Browser APIs
- **Canvas** - VGA framebuffer描画
- **IndexedDB** - VM設定・ディスク永続化
- **File API** - ISO読み込み
- **Keyboard Events** - キー入力
- **Pointer Events** - マウス入力
- **Web Workers** - VM実行の別スレッド化（設計上）

## 依存ライブラリ

| ライブラリ | 用途 |
|-----------|------|
| react, react-dom | UIレンダリング |
| uuid | VM ID生成 |
| tailwindcss | スタイリング |
| wasm-bindgen (Rust) | Rust↔JS連携 |
| js-sys (Rust) | JS APIアクセス |
| web-sys (Rust) | Web APIアクセス |

## プロジェクト構造

```
webvirtualbox/
├── src/
│   ├── App.tsx              # メインアプリケーション
│   ├── main.tsx             # エントリーポイント
│   ├── index.css            # グローバルスタイル
│   ├── types.ts             # 型定義
│   ├── components/
│   │   ├── VMManager.tsx    # VM一覧コンポーネント
│   │   ├── VMConsole.tsx    # VM実行コンソール
│   │   ├── VMCreateDialog.tsx # VM作成ダイアログ
│   │   └── VMSettings.tsx   # VM設定ダイアログ
│   ├── workers/
│   │   ├── vm-worker.ts     # Web Worker (設計)
│   │   └── vm-core.ts       # TypeScript VM Core
│   └── storage/
│       └── vm-storage.ts    # IndexedDB ストレージ
│
├── vm-core/                 # Rust VM Core
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # WASM bindings
│       ├── cpu/
│       │   ├── mod.rs       # CPU構造体
│       │   ├── registers.rs # レジスタ実装
│       │   ├── decoder.rs   # 命令デコード
│       │   ├── executor.rs  # 命令実行
│       │   ├── flags.rs     # フラグレジスタ
│       │   └── instructions.rs # 命令定義
│       ├── memory/
│       │   └── mod.rs       # 仮想メモリ
│       ├── devices/
│       │   ├── mod.rs       # デバイスマネージャ
│       │   ├── vga.rs       # VGAデバイス
│       │   ├── keyboard.rs  # キーボード
│       │   ├── pic.rs       # 割り込みコントローラ
│       │   └── pit.rs       # プログラマブルインターバルタイマ
│       ├── storage/
│       │   └── mod.rs       # ディスク/ISO管理
│       ├── display/
│       │   └── mod.rs       # ディスプレイ管理
│       ├── input/
│       │   └── mod.rs       # 入力管理
│       └── machine/
│           └── mod.rs       # 仮想マシン全体
│
├── index.html
├── package.json
├── vite.config.js
├── tsconfig.json
└── README.md
```

## 実装されている機能

### CPU エミュレーション
- x86 Real Mode命令セット
- 汎用レジスタ (EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP)
- セグメントレジスタ (CS, DS, ES, SS, FS, GS)
- フラグレジスタ (CF, PF, AF, ZF, SF, TF, IF, DF, OF)
- 命令フェッチ・デコード・実行サイクル

### 対応命令
- データ転送: MOV, PUSH, POP, XCHG, LEA, LES, LDS
- 演算: ADD, SUB, INC, DEC, CMP, XOR, AND, OR, TEST
- 分岐: JMP, Jcc (JZ, JNZ, JC, JNC, JS, JNS, JBE, JA), CALL, RET
- 文字列: MOVSB, MOVSW, STOSB, STOSW, REP
- I/O: IN, OUT
- 割り込み: INT, IRET, CLI, STI
- 論理シフト: ROL, ROR, SHL, SHR, SAR
- その他: NOP, HLT, CBW, CWD, MUL, DIV, LOOP, SETcc

### 仮想デバイス
- **VGA** - テキストモード (80x25)、グラフィックモード
- **PIC** (8259A) - マスタ/スレーブ割り込みコントローラ
- **PIT** (8254) - プログラマブルインターバルタイマ
- **Keyboard** (8042) - PS/2キーボードコントローラ

### BIOS
- 最小限のBIOS実装
- INT 10h (ビデオサービス)
- INT 13h (ディスクサービス)
- INT 16h (キーボードサービス)
- INT 19h (ブートストラップ)
- INT 12h (メモリサイズ)
- INT 15h (システムサービス)

### ストレージ
- 仮想ハードディスク (RAW形式)
- 仮想CD-ROM (ISO)
- ブートセクタ読み込み

### 表示
- 640x480 VGA framebuffer
- Canvas へのリアルタイム描画
- テキストモード文字レンダリング
- 16色パレット

### 入力
- キーボード (Set 1 scancode)
- マウス (座標変換、ボタン状態)

### 永続化
- IndexedDB によるVM設定保存
- リロード後の状態復元

## 使い方

1. 「+ New」ボタンで新しいVMを作成
2. 名前、OSタイプ、CPU、メモリ、ディスクサイズを設定
3. ISOファイルを選択（オプション）
4. VM一覧からVMを選択
5. 「Start」ボタンでVMを起動
6. Canvas上でゲストOSの画面が表示される
7. キーボード・マウスで操作可能

## Rust/WASMビルド

Rust VM CoreをWASMにビルドする場合：

```bash
cd vm-core
wasm-pack build --target web
```

ビルドされたWASMモジュールをTypeScript側からロードすることで、
より高速なCPUエミュレーションが利用可能です。

## セキュリティ

- ゲストOSはブラウザのサンドボックス内で動作
- ホストOSのファイルへのアクセス不可
- サーバーへのデータ送信なし
- すべての処理がブラウザ内で完結

## 制限事項

- 最初はReal Mode（16bit）のみ対応
- メモリは最大16MB（ブラウザの制約）
- ネットワーク機能は未実装
- サウンド機能は未実装
- 32bit/64bit プロテクテッドモードは段階的に実装中

## ライセンス

MIT License
