# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**[iced](https://github.com/iced-rs/iced) 的 SwiftUI/Jetpack Compose 风格 modifier API**

链式调用实现样式、布局、交互和组件专属属性 -- 一个流式 API 搞定一切。

> [English](../README.md)

## 快速开始

```toml
[dependencies]
iced_modifier = "0.4"
```

```rust
use iced_modifier::prelude::*;

// 直接链式调用 -- SwiftUI 风格
Text::new("Hello")
    .font_size(16)
    .text_align(TextAlign::Center)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing 直接链式调用
column![
    Text::new("项目 A").font_size(14).padding(8),
    Text::new("项目 B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Hex 颜色 -- 无需 Color::from_rgb()
Text::new("提醒")
    .background_color("#FF5733")
    .text_color("#FFF")
    .border_color([1.0, 0.0, 0.0])

// 带样式的 Button
Button::new(Text::new("提交").font_size(14).color("#FFFFFF"))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## 为什么选择 iced_modifier?

```rust
// 之前: 冗长的 Container 包装 + 样式闭包
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// 之后: 直接链式调用
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## 组件包装器

支持 12 个主要 iced 组件的 modifier 直接链式调用:

| 组件 | 模式 | 组件专属方法 |
|------|------|-------------|
| `Text` | A (-> InteractiveText) | `font_size`, `size`, `font`, `color`, `text_width`, `text_height` |
| `Image` | A (-> InteractiveImage) | `opacity`, `rotation`, `content_fit`, `scale` |
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

**模式 A**: 无 Message 泛型。调用交互方法(`.on_press()`)时转换为交互类型。
**模式 B**: 因子组件/回调而拥有 Message 泛型。可直接使用交互方法。

## 核心类型

| 类型 | 说明 |
|------|------|
| `Text`, `Column`, `Row`, ... | 内置 modifier 的组件包装器。通过直接链式调用应用样式/布局/交互 |
| `Modifier` | 可复用的样式/布局规格。无需泛型即可存储/组合 |
| `Interactor<M>` | Modifier + 交互处理器 |
| `ModifyBase` trait | 提供 40+ 链式方法(组件包装器、Modifier、Interactor 均实现) |

## 功能列表

### 样式

| 方法 | 说明 | Compose 对应 | SwiftUI 对应 |
|------|------|-------------|-------------|
| `.background_color()` | 背景色 (Color, hex, 数组) | `.background()` | `.background()` |
| `.border()` | 边框 (Border, 元组, 半径) | `.border()` | `.border()` |
| `.corner_radius()` | 圆角 | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | 阴影 (Shadow, 元组, 模糊) | `.shadow()` | `.shadow()` |
| `.text_color()` | 文字颜色 (Color, hex, 数组) | `color` 参数 | `.foregroundColor()` |
| `.text_align()` | 文本对齐 (Start, Center, End, Justify) | `textAlign` | `.multilineTextAlignment()` |

### 颜色格式

所有颜色方法 (`.background_color()`, `.text_color()`, `.border_color()`, `.color()`) 支持以下格式:

| 格式 | 示例 |
|------|------|
| `Color` | `Color::WHITE`, `Color::from_rgb(1.0, 0.5, 0.0)` |
| Hex `&str` | `"#FF5733"`, `"#FFF"`, `"3388FF"`, `"#FF573380"` (RGBA) |
| `[f32; 3]` | `[1.0, 0.5, 0.0]` (RGB, 0.0-1.0) |
| `[f32; 4]` | `[1.0, 0.5, 0.0, 0.8]` (RGBA, 0.0-1.0) |

### 边框格式

`.border()` 支持的格式:

| 格式 | 示例 |
|------|------|
| `Border` | `Border { color: Color::RED, width: 1.0, radius: 8.0.into() }` |
| `f32` | `8.0` (仅圆角半径) |
| `(Color, f32)` | `(Color::BLACK, 1.0)` (颜色 + 宽度) |
| `(&str, f32)` | `("#000", 1.0)` (hex 颜色 + 宽度) |
| `(Color, f32, f32)` | `(Color::BLACK, 1.0, 8.0)` (颜色 + 宽度 + 半径) |
| `(&str, f32, f32)` | `("#CCC", 1.0, 8.0)` (hex + 宽度 + 半径) |

### 阴影格式

`.shadow()` 支持的格式:

| 格式 | 示例 |
|------|------|
| `Shadow` | `Shadow { color: Color::BLACK, offset: Vector::new(0.0, 4.0), blur_radius: 8.0 }` |
| `f32` | `8.0` (仅模糊半径, 黑色, 无偏移) |
| `(f32, f32, f32)` | `(0.0, 4.0, 8.0)` (x, y, 模糊) |
| `(f32, f32, f32, Color)` | `(0.0, 4.0, 8.0, Color::BLACK)` |
| `(f32, f32, f32, &str)` | `(0.0, 4.0, 8.0, "#00000040")` (hex 颜色) |

### 布局

| 方法 | 说明 | Compose 对应 | SwiftUI 对应 |
|------|------|-------------|-------------|
| `.padding()` | 内边距 | `.padding()` | `.padding()` |
| `.margin()` | 外边距 | N/A | N/A |
| `.width()` / `.height()` | 尺寸设定 | `.size()` | `.frame()` |
| `.fill_width()` | 填满可用宽度 | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | 按比例填充 | `.weight()` | N/A |
| `.center()` | 双轴居中 | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | 顶部对齐 + 设置高度 | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | 底部对齐 + 设置高度 | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | 最大尺寸 | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | 溢出裁剪 | `.clip()` | `.clipped()` |

### 交互

| 方法 | 说明 | Compose 对应 | SwiftUI 对应 |
|------|------|-------------|-------------|
| `.on_press(msg)` | 点击处理器 | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | 释放处理器 | — | — |
| `.on_double_click(msg)` | 双击 | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | 右键点击 | — | `.contextMenu {}` |
| `.on_enter(msg)` | 悬停进入 | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | 悬停离开 | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | 滚轮事件 | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | 鼠标移动追踪 | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | 光标样式 | `.pointerInput()` | `.cursor()` |

### 附加功能

| 方法 | 说明 | Compose 对应 | SwiftUI 对应 |
|------|------|-------------|-------------|
| `.tooltip_text()` | 提示框 | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | 提示框与内容的间距 | — | — |
| `.tooltip_padding()` | 提示框内边距 | — | — |
| `.tooltip_snap()` | 视口内提示框吸附 | — | — |
| `.scrollable()` | 纵向滚动 | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | 横向滚动 | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | 双向滚动 | — | — |
| `.scroll_anchor_bottom()` | 从底部开始滚动 | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | 从右侧开始滚动 | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | 滚动条与内容的间距 | — | — |
| `.scrollable_id()` | 滚动组件 ID | — | — |
| `.hidden(bool)` | 可见性切换 | `AnimatedVisibility` | `.hidden()` |
| `.id()` | 组件 ID | `.testTag()` | `.id()` |
| `.font_size()` | 字体大小 (组件包装器) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | 子元素间距 (组件包装器) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### 组合与复用

```rust
// 可复用的样式函数 (使用 Modifier)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// 用 .modify() 应用 (向后兼容)
text("Hello").modify(card_style())

// 条件修饰符
Text::new("状态")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// 顺序依赖的分层
Text::new("分层")
    .padding(20)
    .layer()  // 刷新当前层
    .background_color(Color::RED)  // 应用在 padding 外部
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | 说明 |
|---------|-------|------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap 图标字体, `icon_label()` 辅助函数 |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` 扩展 |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | `iced_anim` re-export |
| `image` | iced (image feature) | `Image` / `InteractiveImage` 组件包装器 |
| `all` | 以上全部 | 启用所有功能 |

## 运行示例

```bash
cargo run --example basic
cargo run --example showcase
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## 从 v0.1 迁移

- `use iced::widget::text` -> 移除, 使用 prelude 中的 `text` (组件包装器)
- `use iced::widget::{column, row}` -> 使用 `use iced_modifier::{column, row}` 宏
- `.modify(Modifier::new()...)` -> 仍然有效, 但推荐直接链式调用
- `Extras::tooltip_text` 字段 -> 更名为 `Extras::tooltip` (类型变为 `TooltipConfig`)
- `Extras::scrollable` 字段 -> 类型从 `ScrollDirection` 变为 `ScrollConfig`

## 兼容性

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **平台**: iced 支持的所有平台 (macOS, Windows, Linux, Web)

## 许可证

MIT 许可证。详情请参阅 [LICENSE](../LICENSE)。
