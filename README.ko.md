# SnapTranslate

<h3 align="center">미니멀 · 효율적 · 프라이버시 중심 데스크톱 스크린샷 번역 도구</h3>

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

## 소개

**SnapTranslate**는 개발자와 외국어 학습자를 위한 데스크톱 스크린샷 번역 도구입니다. 화면의 원하는 영역을 선택하면 즉시 OCR 문자 인식 + AI 번역을 수행하고, 번역 결과를 우측 패널에 표시하여 원문과 번역문을 한눈에 비교할 수 있습니다.

**핵심 철학:** 원클릭 영역 선택 → 원위치 고정 → 패널 번역

> 스크린샷이 원래 위치에 고정되고, 번역은 우측 패널에 표시됩니다 — 팝업, 화면 전환, 작업 흐름 중단이 없습니다.

---

## 기능 목록

| 기능 | 설명 |
|------|------|
| **영역 선택 번역** | 전역 단축키 `Ctrl+Alt+L`로 오버레이 실행, 원하는 영역 드래그 선택, 스크린샷을 원래 위치에 자동 고정 |
| **클립보드 고정** | `Ctrl+Alt+P`로 시스템 클립보드의 이미지를 데스크톱에 붙여넣고 번역 |
| **텍스트 번역** | `Ctrl+Alt+M`으로 간결한 텍스트 번역 창 열기, 번역 대상 언어 커스터마이즈 지원, `Ctrl+Enter`로 빠른 번역 |
| **로컬 OCR** | Tesseract 오프라인 엔진 내장, 중국어(간체), 영어, 일본어 다국어 지원, 로컬 스마트 자동 언어 감지 지원 — 인터넷 없이도 작동 |
| **AI 번역** | OpenAI 호환 API 지원 (모델과 키는 직접 준비), AI 환경과 직접 연결 |
| **스마트 번역 캐시** | 반복 콘텐츠는 기록을 자동 조회, 캐시 히트 시 API 호출 없이 즉시 결과 표시 |
| **원위치 고정 창** | 스크린샷을 캡처 위치에 고정, 우측 번역 패널 높이 조절 가능, 다크 테마로 몰입감 제공 |
| **원문/번역 전환** | 원클릭으로 스크린샷 원문과 AI 번역 결과 전환, 대조 학습에 편리 |
| **원클릭 복사** | 원문 또는 번역문을 시스템 클립보드에 복사 |
| **번역 기록** | 모든 번역 기록을 로컬 SQLite 데이터베이스에 자동 저장, 보기/복사/삭제/비우기 지원 |
| **이중 언어 UI** | 간체 중국어 / 영어 인터페이스, 시스템 언어 자동 감지 지원, 즉시 전환 가능 |
| **프라이버시 및 보안** | 스크린샷과 텍스트는 모두 로컬에서 처리, 번역 요청만 직접 설정한 API와 통신 — **텔레메트리나 데이터 업로드 전혀 없음** |
| **자동 시작** | 부팅 시 자동 시작 옵션 설정 가능, 언제든지 사용 가능 |

---

## 사용 방법

### 1. AI API 설정

첫 사용 시 시스템 트레이 아이콘을 우클릭 → **설정**에서 다음을 입력:

- **API 주소**: OpenAI 호환 형식의 모든 API 엔드포인트
- **API 키**: OS 자격 증명 관리자를 통해 안전하게 저장, 디스크에 기록되지 않음
- **모델 이름**: 예: `gpt-4o`, `deepseek-chat` 등
- **번역 대상 언어**: 중국어, 영어, 일본어, 프랑스어 등 9개 언어 지원

### 2. 기본 조작

```
Ctrl+Alt+L 누르기          영역 선택, 스크린샷을 원래 위치에 고정
                              ↓
"번역" 버튼 클릭            OCR + AI 번역, 결과를 우측 패널에 표시
                              ↓
"번역문 복사"              번역문을 클립보드에 복사
                              ↓
동일 내용 다음 캡처        자동 캐시 히트, 결과 즉시 표시

Ctrl+Alt+P 누르기          클립보드 이미지를 데스크톱에 붙여넣고 번역
Ctrl+Alt+M 누르기          텍스트 번역 창 열기, 직접 입력하여 번역
```

### 3. 고정 창 조작

| 조작 | 위치 | 설명 |
|------|------|------|
| 번역 | 컨트롤 바 | 원클릭 OCR + AI 번역 |
| 다시 번역 | 컨트롤 바 | 캐시를 건너뛰고 강제 재번역 |
| 원문/번역문 복사 | 컨트롤 바 | 원클릭으로 클립보드에 복사 |
| 원문/번역 전환 | 컨트롤 바 | 번역 전후 내용 비교 표시 |
| 창 이동 | 창 제목 영역 | 버튼 영역 제외하고 자유롭게 드래그 |
| 번역 패널 늘리기 | 패널 가장자리 | 우측 패널 높이 조절 가능 |
| 닫기 | 이미지 영역 더블클릭 | 고정 창 빠르게 닫기 |

---

## 스크린샷 미리보기

> 다음은 애플리케이션 인터페이스 미리보기입니다 (프로젝트에는 완전한 로고 디자인 페이지 `logo-design.html`이 포함되어 있습니다):

| 모듈 | 미리보기 |
|------|---------|
| **선택 오버레이** | 반투명 다크 마스크 + 흰 점선 선택 상자 + 크기 표시 |
| **고정 창** | 스크린샷을 원래 위치에 최상단 표시 + 하단 컨트롤 바 + 우측 번역 패널 |
| **설정 페이지** | Naive UI 다크 테마, 카테고리별 설정: 언어/일반/API/번역/단축키 |
| **기록** | 썸네일 목록 + 번역 요약 + 조작 버튼 |
| **텍스트 번역** | 화면 하단 중앙에 항상 위에 표시, 간결한 이중 컬럼 구성 |

> 실제 애플리케이션을 실행하여 모든 인터페이스를 직접 경험할 수 있습니다.

---

## 다운로드 및 설치

### 직접 다운로드

[Releases](https://github.com/XuMingKe-06/SanpTranslate/releases) 페이지에서 해당 플랫폼의 최신 설치 프로그램을 다운로드:

| 플랫폼 | 형식 |
|--------|------|
| Windows 10+ | `.msi` / `.exe` |
| macOS 12+ | `.dmg` |
| Linux (x86\_64) | `.deb` / `.AppImage` |

### 시스템 요구 사항

- **Windows**: Windows 10 (1803+), WebView2 (시스템 기본 포함)
- **macOS**: macOS 12+, WebKit (시스템 기본 포함), Homebrew를 통해 Tesseract 및 언어 데이터 패키지 설치 필요:
  ```bash
  brew install tesseract tesseract-lang
  ```
- **Linux**: X11/Wayland 지원, WebKitGTK 필요, Tesseract OCR 엔진 및 관련 언어 팩 설치 필요 (자동 감지 및 단일 언어 모드 모두 해당 `.traineddata` 파일 필요):
  - **Ubuntu / Debian**:
    ```bash
    sudo apt update
    # Tesseract 엔진 및 중국어 간체, 영어, 일본어 언어 팩 설치
    sudo apt install tesseract-ocr tesseract-ocr-chi-sim tesseract-ocr-eng tesseract-ocr-jpn
    ```
  - **Arch Linux**:
    ```bash
    sudo pacman -S tesseract tesseract-data-chi_sim tesseract-data-eng tesseract-data-jpn
    ```

---

## 기술 스택

| 계층 | 기술 |
|------|------|
| 데스크톱 프레임워크 | [Tauri 2.x](https://v2.tauri.app/) |
| 프론트엔드 프레임워크 | [Vue 3.5](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite 6](https://vitejs.dev/) |
| UI 컴포넌트 라이브러리 | [Naive UI](https://www.naiveui.com/) (다크 테마) |
| 백엔드 언어 | [Rust](https://www.rust-lang.org/) (2021 edition) |
| 상태 관리 | [Pinia 3](https://pinia.vuejs.org/) |
| 라우팅 | [Vue Router 5](https://router.vuejs.org/) |
| 국제화 | [vue-i18n 11](https://vue-i18n.intlify.dev/) |
| 화면 캡처 | [xcap](https://crates.io/crates/xcap) |
| OCR | Tesseract CLI (중국어, 영어, 일본어 다국어 지원, 로컬 스마트 자동 언어 감지 지원) |
| AI 번역 | HTTP (reqwest) → OpenAI 호환 API |
| 데이터베이스 | SQLite ([rusqlite](https://crates.io/crates/rusqlite)) |
| 보안 저장소 | [keyring](https://crates.io/crates/keyring) (OS 자격 증명 관리자) |
| 전역 단축키 | [tauri-plugin-global-shortcut](https://github.com/tauri-apps/tauri-plugin-global-shortcut) |
| 클립보드 | [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/tauri-plugin-clipboard-manager) |
| 자동 시작 | [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart) |

---

## 소스 코드에서 빌드

### 환경 준비

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) >= 1.85
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### 빌드 단계

```bash
# 1. 저장소 클론
git clone https://github.com/XuMingKe-06/SanpTranslate.git
cd SnapTranslate

# 2. 프론트엔드 의존성 설치
npm install

# 3. 개발 모드로 실행 (Vite HMR + Tauri)
npm run tauri dev

# 4. 프로덕션 빌드
npm run tauri build
```

빌드 결과물은 `src-tauri/target/release/bundle/` 디렉토리에 생성됩니다.

---

## 설정 파일 위치

| 내용 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 설정 파일 | `%APPDATA%\SnapTranslate\config\config.toml` | `~/Library/Application Support/SnapTranslate/config/config.toml` | `~/.config/SnapTranslate/config/config.toml` |
| 기록 데이터베이스 | `%APPDATA%\SnapTranslate\data\history.db` | `~/Library/Application Support/SnapTranslate/data/history.db` | `~/.local/share/SnapTranslate/data/history.db` |

> **API 키**는 설정 파일에 저장되지 않으며, OS 자격 증명 관리자(Windows Credential Manager / macOS Keychain / Linux Secret Service)를 통해 안전하게 관리됩니다.

---

## 프로젝트 구조

```
SnapTranslate/
├── src/                          # 프론트엔드 소스 (Vue 3 + TypeScript)
│   ├── components/               #   공유 컴포넌트
│   │   ├── ControlBar.vue        #     고정 컨트롤 바 (번역/복사/전환)
│   │   ├── HistoryItem.vue       #     기록 항목
│   │   └── ShortcutInput.vue     #     단축키 캡처 입력
│   ├── views/                    #   페이지 뷰
│   │   ├── OverlayView.vue       #     전체 화면 선택 오버레이 (Canvas)
│   │   ├── PinView.vue           #     고정 창 (스크린샷 + 번역 패널)
│   │   ├── SettingsView.vue      #     설정 페이지 (Naive UI)
│   │   ├── HistoryView.vue       #     기록 페이지
│   │   └── TextTranslateView.vue #     텍스트 번역 창
│   ├── stores/                   #   Pinia 상태 관리
│   │   ├── configStore.ts        #     설정 상태
│   │   ├── pinStore.ts           #     고정 상태
│   │   └── historyStore.ts       #     기록 상태
│   ├── i18n/                     #   국제화
│   │   ├── index.ts              #     vue-i18n 설정
│   │   └── locales/
│   │       ├── zh-CN.ts          #     중국어 언어 팩
│   │       └── en-US.ts          #     영어 언어 팩
│   ├── styles/                   #   전역 스타일
│   │   ├── variables.css         #     CSS 사용자 정의 속성
│   │   └── global.css            #     전역 리셋
│   ├── utils/                    #   유틸리티 함수
│   │   ├── tauri.ts              #     Tauri 명령 바인딩 + 인터페이스 정의
│   │   └── logger.ts             #     구조화된 로깅
│   ├── router/
│   │   └── index.ts              #   Vue Router (5개 라우트)
│   └── main.ts                   #   애플리케이션 진입점
├── src-tauri/                    # Rust 백엔드
│   ├── src/
│   │   ├── capture/mod.rs        #   캡처 모듈 (xcap 래퍼)
│   │   ├── ocr/mod.rs            #   OCR 모듈 (Tesseract CLI)
│   │   ├── translate/mod.rs      #   번역 모듈 (AI API + 캐시)
│   │   ├── config/               #   설정 관리 (TOML + keyring)
│   │   ├── history/mod.rs        #   기록 (SQLite CRUD)
│   │   ├── clipboard/mod.rs      #   클립보드 읽기/쓰기
│   │   ├── hotkey/mod.rs         #   전역 단축키 등록
│   │   ├── window/mod.rs         #   창 관리 (싱글톤/멀티 인스턴스)
│   │   ├── tray/mod.rs           #   시스템 트레이 메뉴
│   │   ├── commands.rs           #   21개 Tauri 명령
│   │   ├── error.rs              #   통합 오류 타입
│   │   ├── lib.rs                #   Setup 진입점
│   │   └── main.rs               #   Main 함수
│   └── resources/tesseract/      #   Tesseract OCR 오프라인 데이터
├── docs/                         # 프로젝트 문서
│   ├── SRS.md                    #   소프트웨어 요구사항 명세서
│   ├── ARCHITECTURE.md           #   아키텍처 설계 문서
│   ├── HLD.md                    #   상위 수준 설계
│   ├── DLD.md                    #   상세 설계
│   ├── TEST_PLAN.md              #   테스트 계획
│   └── TEST_DESIGN.md            #   테스트 설계 명세서
├── package.json
├── CLAUDE.md                     # 개발 가이드
└── LICENSE                       # MIT License
```

---

## 데이터 흐름

```
[사용자가 전역 단축키 누름]
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    캡처 모듈                               │
│  xcap 전체 화면 캡처 → CachedScreenStore에 캐시            │
│  → 전체 화면 오버레이 창 생성 → 드래그 선택                │
│  → 영역 자르기 → store_pin_image → 고정 창 생성            │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    고정 창                                 │
│  PinView가 이미지 가져오기 → 스크린샷 + 컨트롤 바 표시     │
│  사용자가 "번역" 클릭 → translate_image 호출              │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                OCR + 번역 파이프라인                        │
│  ① Tesseract 오프라인 인식 → 텍스트 + 좌표 추출            │
│  ② 기록 캐시 확인 ──→ 히트? ──→ 캐시 결과 반환            │
│                    │                                       │
│                  미스                                     │
│                    │                                       │
│  ③ AI API 호출 → 번역 분석 → 좌표 정렬                   │
│  ④ SQLite 기록에 비동기 저장                             │
└──────────────────────────────────────────────────────────┘
        │
        ▼
[번역 결과를 우측 패널에 표시, 원문/번역 전환, 복사, 늘리기 지원]
```

---

## 디자인 철학

- **프라이버시 우선:** OCR은 로컬에서 실행 — 스크린샷이 제3자 서비스에 업로드될 위험이 없습니다. 번역은 직접 설정한 API 엔드포인트와만 통신, 텔레메트리나 추적이 전혀 없습니다
- **캡처 후 즉시:** 스크린샷은 번역 전에 원래 위치에 고정 — 새 창이 열리지 않으며 현재 작업 흐름을 방해하지 않습니다
- **오프라인에서도 안정적:** 인터넷이 없어도 스크린샷과 고정 기능이 완전히 작동하며, OCR은 완전히 오프라인에서 실행됩니다
- **캐시로 효율성 향상:** 동일한 콘텐츠는 기록 캐시를 조회하여 즉시 결과 표시, API 호출 비용 절감
- **가볍고 자족적:** Tauri 기반으로 설치 프로그램이 작고 메모리 소비가 낮으며, Rust 백엔드가 고성능과 저전력을 보장합니다

---

## 라이선스

[MIT License](LICENSE)

Copyright © 2026 XuMingKe
