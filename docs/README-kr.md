# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**[iced](https://github.com/iced-rs/iced)를 위한 SwiftUI/Jetpack Compose 스타일 modifier API**

스타일, 레이아웃, 인터랙션, 위젯 고유 속성을 하나의 체이닝 API로 제공합니다.

> [English](../README.md)

## 빠른 시작

```toml
[dependencies]
iced_modifier = "0.2"
```

```rust
use iced_modifier::prelude::*;

// 직접 체이닝 — SwiftUI 스타일
Text::new("Hello")
    .font_size(16)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing 직접 체이닝
column![
    Text::new("항목 A").font_size(14).padding(8),
    Text::new("항목 B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// 스타일된 Button
Button::new(Text::new("제출").font_size(14).color(Color::WHITE))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## 왜 iced_modifier인가?

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

// 이후: 직접 체이닝
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## 위젯 래퍼

12개 주요 iced 위젯을 modifier 직접 체이닝으로 지원합니다:

| 위젯 | 패턴 | 위젯 고유 메서드 |
|------|------|----------------|
| `Text` | A (→ InteractiveText) | `font_size`, `size`, `font`, `color`, `text_width`, `text_height` |
| `Image` | A (→ InteractiveImage) | `opacity`, `rotation`, `content_fit`, `scale` |
| `Column` | B | `spacing`, `push`, `extend`, `column![]` |
| `Row` | B | `spacing`, `push`, `extend`, `row![]` |
| `Button` | B | `on_press`, `on_press_maybe`, `button_padding` |
| `TextInput` | B | `font_size`, `on_input`, `on_submit`, `on_paste`, `secure` |
| `TextEditor` | B | `font_size`, `on_action`, `wrapping`, `editor_padding` |
| `Checkbox` | B | `on_toggle`, `label`, `check_size`, `check_spacing`, `text_size` |
| `Radio` | B | `radio_size`, `radio_spacing`, `text_size` |
| `Toggler` | B | `on_toggle`, `label`, `toggler_size`, `toggler_spacing`, `text_size` |
| `Slider` | B | `step`, `shift_step`, `on_release`, `slider_width`, `slider_height` |
| `PickList` | B | `placeholder`, `text_size`, `menu_height`, `on_open`, `on_close` |

**패턴 A**: Message 제네릭 없음. 인터랙션 메서드(`.on_press()`) 호출 시 인터랙티브 타입으로 전환.
**패턴 B**: 자식/콜백으로 인해 Message 제네릭 보유. 인터랙션 직접 사용 가능.

## 핵심 타입

| 타입 | 설명 |
|------|------|
| `Text`, `Column`, `Row`, ... | modifier 내장 위젯 래퍼. 직접 체이닝으로 스타일/레이아웃/인터랙션 적용 |
| `Modifier` | 재사용 가능한 스타일/레이아웃 명세. 제네릭 없이 저장/합성 가능 |
| `Interactor<M>` | Modifier + 인터랙션 핸들러 |
| `ModifyBase` trait | 40+ 체이닝 메서드 제공 (위젯 래퍼, Modifier, Interactor 모두 구현) |

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
| `.margin()` | 외부 여백 | N/A | N/A |
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
| `.font_size()` | 폰트 크기 (위젯 래퍼) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | 자식 간격 (위젯 래퍼) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### 합성 & 재사용

```rust
// 재사용 가능한 스타일 함수 (Modifier 사용)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// .modify()로 적용 (하위 호환)
text("Hello").modify(card_style())

// 조건부 modifier
Text::new("상태")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// 순서 의존적 레이어링
Text::new("레이어")
    .padding(20)
    .layer()  // 현재 레이어 flush
    .background_color(Color::RED)  // padding 바깥에 적용
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | 크레이트 | 설명 |
|---------|---------|------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap 아이콘 폰트, `icon_label()` 헬퍼 |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` 확장 |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | `iced_anim` re-export |
| `image` | iced (image feature) | `Image` / `InteractiveImage` 위젯 래퍼 |
| `all` | 위 전부 | 모든 기능 활성화 |

## 예제 실행

```bash
cargo run --example basic
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## v0.1에서 마이그레이션

- `use iced::widget::text` → 제거, prelude의 `text` 사용 (위젯 래퍼)
- `use iced::widget::{column, row}` → `use iced_modifier::{column, row}` 매크로 사용
- `.modify(Modifier::new()...)` → 여전히 동작하지만 직접 체이닝 권장
- `Extras::tooltip_text` 필드 → `Extras::tooltip`으로 변경 (타입: `TooltipConfig`)
- `Extras::scrollable` 필드 → 타입 `ScrollDirection` → `ScrollConfig`으로 변경

## 호환성

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **플랫폼**: iced가 지원하는 모든 플랫폼 (macOS, Windows, Linux, Web)

## 라이선스

MIT 라이선스. 자세한 내용은 [LICENSE](../LICENSE)를 참조하세요.
