# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**[iced](https://github.com/iced-rs/iced)를 위한 SwiftUI/Jetpack Compose 스타일 modifier API**

스타일, 레이아웃, 인터랙션, 툴팁, 스크롤을 하나의 체이닝 API로 제공합니다.

> [English](../README.md)

## 빠른 시작

```toml
[dependencies]
iced_modifier = "0.1"
```

```rust
use iced::widget::text;
use iced::Color;
use iced_modifier::prelude::*;

// 순수 스타일링 — Element를 직접 반환, .into() 불필요
let card = text("Hello").modify(
    Modifier::new()
        .padding(16)
        .background_color(Color::WHITE)
        .corner_radius(12)
);

// 인터랙션 포함 (클릭, 호버, 커서)
let button = text("클릭하세요").modify(
    Modifier::new()
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .on_press(Message::Clicked)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false))
        .cursor(mouse::Interaction::Pointer)
);
```

## 왜 iced_modifier인가?

iced의 현재 API는 일반적인 스타일링에도 장황한 Container 래핑이 필요합니다:

```rust
// 이전: 장황한 Container 래핑 + 스타일 클로저
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// 이후: iced_modifier로 한 줄
text("Hello").modify(
    Modifier::new().padding(16).background_color(Color::WHITE).corner_radius(12)
)
```

## 핵심 타입

| 타입 | 설명 |
|------|------|
| `Modifier` | 스타일/레이아웃 명세. 제네릭 없이 저장/재사용/합성 가능 |
| `Interactor<M>` | Modifier + 인터랙션 핸들러. `.on_press()` 호출 시 자동 생성 |
| `Modify` trait | 모든 iced 위젯에 `.modify()` 메서드를 추가하는 확장 트레이트 |
| `ModifyBase` trait | 모든 체이닝 메서드를 제공하는 트레이트 (Modifier, Interactor 공유) |

## 기능 목록

### 스타일

| 메서드 | 설명 | Compose 대응 | SwiftUI 대응 |
|--------|------|-------------|-------------|
| `.background_color()` | 배경색 | `.background()` | `.background()` |
| `.border()` | 테두리 전체 설정 | `.border()` | `.border()` |
| `.corner_radius()` | 모서리 둥글기 | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | 그림자 | `.shadow()` | `.shadow()` |
| `.text_color()` | 텍스트 색상 | `color` 파라미터 | `.foregroundColor()` |

### 레이아웃

| 메서드 | 설명 | Compose 대응 | SwiftUI 대응 |
|--------|------|-------------|-------------|
| `.padding()` | 내부 여백 | `.padding()` | `.padding()` |
| `.margin()` | 외부 여백 | N/A (padding 사용) | N/A |
| `.width()` / `.height()` | 크기 지정 | `.size()` | `.frame()` |
| `.fill_width()` | 가용 너비 채우기 | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | 비율 채우기 | `.weight()` | N/A |
| `.center()` | 양축 중앙 정렬 | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | 상단 정렬 + 높이 설정 | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | 하단 정렬 + 높이 설정 | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | 최대 크기 | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | 오버플로 클리핑 | `.clip()` | `.clipped()` |

### 인터랙션

| 메서드 | 설명 | Compose 대응 | SwiftUI 대응 |
|--------|------|-------------|-------------|
| `.on_press(msg)` | 클릭 핸들러 | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | 릴리즈 핸들러 | — | — |
| `.on_double_click(msg)` | 더블클릭 | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | 우클릭 | — | `.contextMenu {}` |
| `.on_enter(msg)` | 호버 진입 | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | 호버 이탈 | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | 스크롤 휠 이벤트 | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | 마우스 이동 추적 | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | 커서 스타일 | `.pointerInput()` | `.cursor()` |

### 추가 기능

| 메서드 | 설명 | Compose 대응 | SwiftUI 대응 |
|--------|------|-------------|-------------|
| `.tooltip_text()` | 툴팁 | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | 툴팁-콘텐츠 간격 | — | — |
| `.tooltip_padding()` | 툴팁 내부 패딩 | — | — |
| `.tooltip_snap()` | 뷰포트 내 툴팁 스냅 | — | — |
| `.scrollable()` | 세로 스크롤 | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | 가로 스크롤 | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | 양방향 스크롤 | — | — |
| `.scroll_anchor_bottom()` | 하단에서 스크롤 시작 | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | 우측에서 스크롤 시작 | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | 스크롤바-콘텐츠 간격 | — | — |
| `.scrollable_id()` | 스크롤 위젯 ID | — | — |
| `.hidden(bool)` | 가시성 토글 | `AnimatedVisibility` | `.hidden()` |
| `.id()` | 위젯 ID | `.testTag()` | `.id()` |

### 합성 & 재사용

```rust
// 재사용 가능한 스타일 함수
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// then()으로 합성
let elevated = card_style().then(Modifier::new().shadow(shadow));

// 조건부 modifier
Modifier::new()
    .padding(10)
    .modify_if(is_error, |m| m.background_color(Color::RED))

// 순서 의존적 레이어링
Modifier::new()
    .padding(20)
    .layer()  // 현재 레이어 flush
    .background_color(Color::RED)  // padding 바깥에 적용
```

## Feature Flags

```toml
# 선택적 기능
iced_modifier = { version = "0.1", features = ["icons", "drag-drop", "animation"] }
```

| Feature | 크레이트 | 설명 |
|---------|---------|------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap 아이콘 폰트, `icon_label()` 헬퍼 |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` 확장 |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | `iced_anim` re-export |
| `all` | 위 전부 | 모든 기능 활성화 |

## 예제 실행

```bash
cargo run --example basic
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## 호환성

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **플랫폼**: iced가 지원하는 모든 플랫폼 (macOS, Windows, Linux, Web)

## 라이선스

MIT 라이선스. 자세한 내용은 [LICENSE](../LICENSE)를 참조하세요.
