# SnapTranslate

<h3 align="center">ミニマル · 効率的 · プライバシー重視のデスクトップスクリーンショット翻訳ツール</h3>

<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" alt="SnapTranslate Logo" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.x-FFC131" alt="Tauri 2.x" />
  <img src="https://img.shields.io/static/v1?label=Rust&message=2024&color=orange" alt="Rust 2024" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vue.js" alt="Vue 3.5" />
  <img src="https://img.shields.io/badge/TypeScript-5.7-3178C6?logo=typescript" alt="TypeScript 5.7" />
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT License" />
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey" alt="Cross Platform" />
</p>

<p align="center">
  <a href="README.zh.md">简体中文</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.en.md">English</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a>
</p>

---

## 概要

**SnapTranslate** は、開発者や語学学習者のためのデスクトップスクリーンショット翻訳ツールです。画面上の任意の領域を選択するだけで、OCR 文字認識 + AI 翻訳を瞬時に実行。翻訳結果は右側パネルに表示され、原文と訳文を並べて一目で確認できます。

**コアコンセプト:** ワンクリックで領域選択 → その場に固定表示 → パネル翻訳

> スクリーンショットは元の位置に固定表示され、翻訳は右側パネルに表示されます — ポップアップ、画面遷移、ワークフローの中断は一切ありません。

---

## 機能一覧

| 機能 | 説明 |
|------|------|
| **領域選択翻訳** | グローバルホットキー `Ctrl+Alt+L` でオーバーレイ起動、任意の領域をドラッグ選択、スクリーンショットを元の位置に自動貼り付け |
| **クリップボード貼り付け** | `Ctrl+Alt+P` でシステムクリップボードの画像をデスクトップに貼り付けて翻訳 |
| **テキスト翻訳** | `Ctrl+Alt+M` でシンプルなテキスト翻訳ウィンドウを開き、翻訳先言語のカスタマイズに対応、`Ctrl+Enter` で素早く翻訳 |
| **ローカル OCR** | Tesseract オフラインエンジンを内蔵、インターネット不要で文字と段落位置を自動認識 |
| **AI 翻訳** | OpenAI 互換 API に対応（モデルとキーは自己管理）、お使いの AI 環境と直接連携 |
| **スマート翻訳キャッシュ** | 同一コンテンツは履歴を自動照合、キャッシュヒット時は API 呼び出しをスキップして瞬時に結果表示 |
| **その場固定ウィンドウ** | スクリーンショットをキャプチャ位置に固定、右側の翻訳パネルは高さ調整可能、ダークテーマで没入感を実現 |
| **原文/翻訳切替** | ワンクリックでスクリーンショットの原文と AI 翻訳結果を切替、対照学習に便利 |
| **ワンクリックコピー** | 原文または翻訳文をシステムクリップボードにコピー |
| **翻訳履歴** | 全翻訳記録をローカル SQLite データベースに自動保存、表示・コピー・削除・クリアに対応 |
| **バイリンガル UI** | 簡体字中国語 / 英語の二言語インターフェース、システム言語自動検出対応、即時切替可能 |
| **プライバシーとセキュリティ** | スクリーンショットとテキストはすべてローカル処理、翻訳リクエストのみ自己管理の API と通信 — **テレメトリーやデータアップロードは一切なし** |
| **自動起動** | 起動時自動起動の設定が可能、いつでも使用可能な状態に |

---

## 使用手順

### 1. AI API の設定

初回使用時は、システムトレイアイコンを右クリック → **設定** で以下を入力：

- **API アドレス**: OpenAI 互換形式の任意の API エンドポイント
- **API キー**: OS の認証情報マネージャーで安全に保存、ディスクには書き込まれません
- **モデル名**: 例：`gpt-4o`、`deepseek-chat` など
- **翻訳先言語**: 中国語、英語、日本語、フランス語など 9 言語に対応

### 2. 基本操作

```
Ctrl+Alt+L を押す         領域を選択、スクリーンショットを元の位置に貼り付け
                               ↓
「翻訳」ボタンをクリック      OCR + AI 翻訳、結果を右側パネルに表示
                               ↓
「翻訳文をコピー」           翻訳文をクリップボードにコピー
                               ↓
同じ内容を次回キャプチャ     自動キャッシュヒット、結果を即時表示

Ctrl+Alt+P を押す         クリップボードの画像をデスクトップに貼り付けて翻訳
Ctrl+Alt+M を押す         テキスト翻訳ウィンドウを開き、直接入力して翻訳
```

### 3. 固定ウィンドウの操作

| 操作 | 場所 | 説明 |
|------|------|------|
| 翻訳 | コントロールバー | ワンクリック OCR + AI 翻訳 |
| 再翻訳 | コントロールバー | キャッシュをスキップして強制再翻訳 |
| 原文/翻訳文をコピー | コントロールバー | ワンクリックでクリップボードにコピー |
| 原文/翻訳切替 | コントロールバー | 翻訳前後の内容を比較表示 |
| ウィンドウ移動 | ウィンドウタイトル領域 | ボタン領域を除き自由にドラッグ |
| 翻訳パネル伸縮 | パネル端 | 右側パネルの高さを調整可能 |
| 閉じる | 画像領域をダブルクリック | 固定ウィンドウをすばやく閉じる |

---

## スクリーンショット

> 以下はアプリケーションインターフェースのプレビューです（プロジェクトには完全なロゴデザインページ `logo-design.html` が付属）：

| モジュール | プレビュー |
|-----------|-----------|
| **選択オーバーレイ** | 半透明ダークマスク + 白点線選択枠 + サイズ表示 |
| **固定ウィンドウ** | スクリーンショットを元の位置に前面表示 + 下部コントロールバー + 右側翻訳パネル |
| **設定ページ** | Naive UI ダークテーマ、カテゴリ別設定：言語/一般/API/翻訳/ショートカットキー |
| **履歴** | サムネイル一覧 + 翻訳サマリー + 操作ボタン |
| **テキスト翻訳** | 画面下部中央に常時前面表示、シンプルな二段構成 |

> 実際のアプリケーションを実行して、すべてのインターフェースを体験できます。

---

## ダウンロードとインストール

### 直接ダウンロード

[Releases](https://github.com/XuMingKe-06/SanpTranslate/releases) ページから対象プラットフォームの最新インストーラーをダウンロード：

| プラットフォーム | 形式 |
|-----------------|------|
| Windows 10+ | `.msi` / `.exe` |
| macOS 12+ | `.dmg` |
| Linux (x86\_64) | `.deb` / `.AppImage` |

### システム要件

- **Windows**: Windows 10 (1803+)、WebView2（システム標準搭載）
- **macOS**: macOS 12+、WebKit（システム標準搭載）
- **Linux**: X11/Wayland 対応、WebKitGTK が必要

---

## 技術スタック

| 階層 | 技術 |
|------|------|
| デスクトップフレームワーク | [Tauri 2.x](https://v2.tauri.app/) |
| フロントエンドフレームワーク | [Vue 3.5](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite 6](https://vitejs.dev/) |
| UI コンポーネントライブラリ | [Naive UI](https://www.naiveui.com/)（ダークテーマ） |
| バックエンド言語 | [Rust](https://www.rust-lang.org/)（2021 edition） |
| 状態管理 | [Pinia 3](https://pinia.vuejs.org/) |
| ルーティング | [Vue Router 5](https://router.vuejs.org/) |
| 国際化 | [vue-i18n 11](https://vue-i18n.intlify.dev/) |
| スクリーンキャプチャ | [xcap](https://crates.io/crates/xcap) |
| OCR | Tesseract CLI（`chi_sim` + `eng` 言語データバンドル） |
| AI 翻訳 | HTTP (reqwest) → OpenAI 互換 API |
| データベース | SQLite ([rusqlite](https://crates.io/crates/rusqlite)) |
| セキュアストレージ | [keyring](https://crates.io/crates/keyring)（OS 認証情報マネージャー） |
| グローバルホットキー | [tauri-plugin-global-shortcut](https://github.com/tauri-apps/tauri-plugin-global-shortcut) |
| クリップボード | [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/tauri-plugin-clipboard-manager) |
| 自動起動 | [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart) |

---

## ソースコードからのビルド

### 環境準備

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) >= 1.85
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### ビルド手順

```bash
# 1. リポジトリをクローン
git clone https://github.com/XuMingKe-06/SanpTranslate.git
cd SnapTranslate

# 2. フロントエンド依存関係をインストール
npm install

# 3. 開発モードで実行（Vite HMR + Tauri）
npm run tauri dev

# 4. プロダクションビルド
npm run tauri build
```

ビルド成果物は `src-tauri/target/release/bundle/` ディレクトリに生成されます。

---

## 設定ファイルの場所

| 内容 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 設定ファイル | `%APPDATA%\SnapTranslate\config\config.toml` | `~/Library/Application Support/SnapTranslate/config/config.toml` | `~/.config/SnapTranslate/config/config.toml` |
| 履歴データベース | `%APPDATA%\SnapTranslate\data\history.db` | `~/Library/Application Support/SnapTranslate/data/history.db` | `~/.local/share/SnapTranslate/data/history.db` |

> **API キー**は設定ファイルには保存されず、OS の認証情報マネージャー（Windows Credential Manager / macOS Keychain / Linux Secret Service）で安全に管理されます。

---

## プロジェクト構造

```
SnapTranslate/
├── src/                          # フロントエンドソース (Vue 3 + TypeScript)
│   ├── components/               #   共有コンポーネント
│   │   ├── ControlBar.vue        #     固定コントロールバー（翻訳/コピー/切替）
│   │   ├── HistoryItem.vue       #     履歴エントリー
│   │   └── ShortcutInput.vue     #     ホットキーキャプチャ入力
│   ├── views/                    #   ページビュー
│   │   ├── OverlayView.vue       #     全画面選択オーバーレイ（Canvas）
│   │   ├── PinView.vue           #     固定ウィンドウ（スクリーンショット＋翻訳パネル）
│   │   ├── SettingsView.vue      #     設定ページ（Naive UI）
│   │   ├── HistoryView.vue       #     履歴ページ
│   │   └── TextTranslateView.vue #     テキスト翻訳ウィンドウ
│   ├── stores/                   #   Pinia 状態管理
│   │   ├── configStore.ts        #     設定状態
│   │   ├── pinStore.ts           #     固定状態
│   │   └── historyStore.ts       #     履歴状態
│   ├── i18n/                     #   国際化
│   │   ├── index.ts              #     vue-i18n 設定
│   │   └── locales/
│   │       ├── zh-CN.ts          #     中国語言語パック
│   │       └── en-US.ts          #     英語言語パック
│   ├── styles/                   #   グローバルスタイル
│   │   ├── variables.css         #     CSS カスタムプロパティ
│   │   └── global.css            #     グローバルリセット
│   ├── utils/                    #   ユーティリティ関数
│   │   ├── tauri.ts              #     Tauri コマンドバインディング＋インターフェース定義
│   │   └── logger.ts             #     構造化ログ
│   ├── router/
│   │   └── index.ts              #   Vue Router（5 ルート）
│   └── main.ts                   #   アプリケーションエントリー
├── src-tauri/                    # Rust バックエンド
│   ├── src/
│   │   ├── capture/mod.rs        #   キャプチャモジュール（xcap ラッパー）
│   │   ├── ocr/mod.rs            #   OCR モジュール（Tesseract CLI）
│   │   ├── translate/mod.rs      #   翻訳モジュール（AI API + キャッシュ）
│   │   ├── config/               #   設定管理（TOML + keyring）
│   │   ├── history/mod.rs        #   履歴（SQLite CRUD）
│   │   ├── clipboard/mod.rs      #   クリップボード読み書き
│   │   ├── hotkey/mod.rs         #   グローバルホットキー登録
│   │   ├── window/mod.rs         #   ウィンドウ管理（シングルトン/マルチインスタンス）
│   │   ├── tray/mod.rs           #   システムトレイメニュー
│   │   ├── commands.rs           #   21 の Tauri コマンド
│   │   ├── error.rs              #   統一エラー型
│   │   ├── lib.rs                #   Setup エントリー
│   │   └── main.rs               #   Main 関数
│   └── resources/tesseract/      #   Tesseract OCR オフラインデータ
├── docs/                         # プロジェクトドキュメント
│   ├── SRS.md                    #   ソフトウェア要件仕様書
│   ├── ARCHITECTURE.md           #   アーキテクチャ設計書
│   ├── HLD.md                    #   概要設計書
│   ├── DLD.md                    #   詳細設計書
│   ├── TEST_PLAN.md              #   テスト計画書
│   └── TEST_DESIGN.md            #   テスト設計仕様書
├── package.json
├── CLAUDE.md                     # 開発ガイド
└── LICENSE                       # MIT License
```

---

## データフロー

```
[ユーザーがグローバルホットキーを押す]
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    キャプチャモジュール                      │
│  xcap 全画面キャプチャ → CachedScreenStore にキャッシュ     │
│  → 全画面オーバーレイウィンドウを作成 → ドラッグ選択        │
│  → 領域を切り抜き → store_pin_image → 固定ウィンドウ作成    │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    固定ウィンドウ                            │
│  PinView が画像を取得 → スクリーンショット＋コントロールバー  │
│  ユーザーが「翻訳」をクリック → translate_image を呼び出し   │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                  OCR + 翻訳パイプライン                      │
│  ① Tesseract オフライン認識 → テキスト＋座標を抽出          │
│  ② 履歴キャッシュを確認 ──→ ヒット？──→ キャッシュ結果を返却│
│                    │                                       │
│                  ミス                                     │
│                    │                                       │
│  ③ AI API を呼び出し → 翻訳を解析 → 座標を調整            │
│  ④ SQLite 履歴に非同期保存                                │
└──────────────────────────────────────────────────────────┘
        │
        ▼
[翻訳結果を右側パネルに表示、原文/翻訳切替、コピー、伸縮に対応]
```

---

## 設計理念

- **プライバシー優先:** OCR はローカルで実行 — スクリーンショットが第三者サービスにアップロードされるリスクはありません。翻訳は自身の API エンドポイントとのみ通信。テレメトリー、トラッキングは一切なし
- **キャプチャしてすぐに:** スクリーンショットは翻訳前にその場に固定 — 新規ウィンドウは開かず、現在のワークフローを中断しません
- **オフラインでも安心:** インターネットがなくてもスクリーンショットと固定機能は完全に動作。OCR は完全オフライン
- **キャッシュで効率化:** 同一コンテンツは履歴キャッシュを照合して瞬時に結果表示、API 呼び出しコストを削減
- **軽量で自己完結:** Tauri ベースでインストーラーは小型、メモリ消費は低く、Rust バックエンドが高性能と低消費電力を保証

---

## ライセンス

[MIT License](LICENSE)

Copyright © 2026 XuMingKe
