# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**[iced](https://github.com/iced-rs/iced) 向け SwiftUI/Jetpack Compose スタイル modifier API**

スタイル、レイアウト、インタラクション、ウィジェット固有プロパティを1つのチェーン API で提供します。

> [English](../README.md)

## クイックスタート

```toml
[dependencies]
iced_modifier = "0.5"
```

```rust
use iced_modifier::prelude::*;

// 直接チェーン -- SwiftUI スタイル
Text::new("Hello")
    .font_size(16)
    .text_align(TextAlign::Center)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing 直接チェーン
column![
    Text::new("項目 A").font_size(14).padding(8),
    Text::new("項目 B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Hex カラー -- Color::from_rgb() 不要
Text::new("アラート")
    .background_color("#FF5733")
    .text_color("#FFF")
    .border_color([1.0, 0.0, 0.0])

// スタイル付き Button
Button::new(Text::new("送信").font_size(14).color("#FFFFFF"))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## なぜ iced_modifier なのか?

```rust
// 以前: 冗長な Container ラッピング + スタイルクロージャ
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// 以後: 直接チェーン
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## ウィジェットラッパー

12 の主要 iced ウィジェットを modifier 直接チェーンでサポート:

| ウィジェット | パターン | ウィジェット固有メソッド |
|-------------|---------|----------------------|
| `Text` | A (-> InteractiveText) | `font_size`, `size`, `font`, `color`, `text_width`, `text_height` |
| `Image` | A (-> InteractiveImage) | `opacity`, `rotation`, `content_fit`, `scale` |
| `Column` | B | `spacing`, `push`, `extend`, `column![]` |
| `Row` | B | `spacing`, `push`, `extend`, `row![]` |
| `Button` | B\* | `on_press`, `on_press_maybe`, `button_padding` |
| `TextInput` | B | `font_size`, `on_input`, `on_submit`, `on_paste`, `secure` |
| `TextEditor` | B | `font_size`, `on_action`, `wrapping`, `editor_padding` |
| `Checkbox` | B | `on_toggle`, `label`, `check_size`, `check_spacing`, `text_size` |
| `Radio` | B | `radio_size`, `radio_spacing`, `text_size` |
| `Toggler` | B | `on_toggle`, `label`, `toggler_size`, `toggler_spacing`, `text_size` |
| `Slider` | B | `step`, `shift_step`, `on_release`, `slider_width`, `slider_height` |
| `PickList` | B | `placeholder`, `text_size`, `menu_height`, `on_open`, `on_close` |

**パターン A**: Message ジェネリックなし。インタラクションメソッド(`.on_press()`)呼び出し時にインタラクティブ型へ遷移。
**パターン B**: 子要素/コールバックにより Message ジェネリック保持。インタラクションを直接使用可能。
**B\***: Button はスタイル属性（`background_color`, `corner_radius`, `border`, `shadow`, `text_color`）を Container ラッピングではなく iced Button に直接適用します。

## コア型

| 型 | 説明 |
|----|------|
| `Text`, `Column`, `Row`, ... | modifier 内蔵ウィジェットラッパー。直接チェーンでスタイル/レイアウト/インタラクションを適用 |
| `Modifier` | 再利用可能なスタイル/レイアウト仕様。ジェネリック不要で保存/合成可能 |
| `Interactor<M>` | Modifier + インタラクションハンドラ |
| `ModifyBase` trait | 40+ チェーンメソッドを提供(ウィジェットラッパー、Modifier、Interactor すべてが実装) |

## 機能一覧

### スタイル

| メソッド | 説明 | Compose 対応 | SwiftUI 対応 |
|---------|------|-------------|-------------|
| `.background_color()` | 背景色 (Color, hex, 配列) | `.background()` | `.background()` |
| `.border()` | ボーダー (Border, タプル, 半径) | `.border()` | `.border()` |
| `.corner_radius()` | 角丸 | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | シャドウ (Shadow, タプル, ブラー) | `.shadow()` | `.shadow()` |
| `.text_color()` | テキスト色 (Color, hex, 配列) | `color` パラメータ | `.foregroundColor()` |
| `.text_align()` | テキスト配置 (Start, Center, End, Justify) | `textAlign` | `.multilineTextAlignment()` |

### カラーフォーマット

すべてのカラーメソッド (`.background_color()`, `.text_color()`, `.border_color()`, `.color()`) は以下のフォーマットに対応:

| フォーマット | 例 |
|------------|-----|
| `Color` | `Color::WHITE`, `Color::from_rgb(1.0, 0.5, 0.0)` |
| Hex `&str` | `"#FF5733"`, `"#FFF"`, `"3388FF"`, `"#FF573380"` (RGBA) |
| `[f32; 3]` | `[1.0, 0.5, 0.0]` (RGB, 0.0-1.0) |
| `[f32; 4]` | `[1.0, 0.5, 0.0, 0.8]` (RGBA, 0.0-1.0) |

### ボーダーフォーマット

`.border()` の対応フォーマット:

| フォーマット | 例 |
|------------|-----|
| `Border` | `Border { color: Color::RED, width: 1.0, radius: 8.0.into() }` |
| `f32` | `8.0` (角丸半径のみ) |
| `(Color, f32)` | `(Color::BLACK, 1.0)` (色 + 幅) |
| `(&str, f32)` | `("#000", 1.0)` (hex 色 + 幅) |
| `(Color, f32, f32)` | `(Color::BLACK, 1.0, 8.0)` (色 + 幅 + 半径) |
| `(&str, f32, f32)` | `("#CCC", 1.0, 8.0)` (hex + 幅 + 半径) |

### シャドウフォーマット

`.shadow()` の対応フォーマット:

| フォーマット | 例 |
|------------|-----|
| `Shadow` | `Shadow { color: Color::BLACK, offset: Vector::new(0.0, 4.0), blur_radius: 8.0 }` |
| `f32` | `8.0` (ブラー半径のみ、黒色、オフセットなし) |
| `(f32, f32, f32)` | `(0.0, 4.0, 8.0)` (x, y, ブラー) |
| `(f32, f32, f32, Color)` | `(0.0, 4.0, 8.0, Color::BLACK)` |
| `(f32, f32, f32, &str)` | `(0.0, 4.0, 8.0, "#00000040")` (hex カラー) |

### レイアウト

| メソッド | 説明 | Compose 対応 | SwiftUI 対応 |
|---------|------|-------------|-------------|
| `.padding()` | 内側余白 | `.padding()` | `.padding()` |
| `.margin()` | 外側余白 | N/A | N/A |
| `.width()` / `.height()` | サイズ指定 | `.size()` | `.frame()` |
| `.fill_width()` | 利用可能な幅を埋める | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | 比率で埋める | `.weight()` | N/A |
| `.center()` | 両軸中央揃え | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | 上揃え + 高さ設定 | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | 下揃え + 高さ設定 | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | 最大サイズ | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | オーバーフロークリッピング | `.clip()` | `.clipped()` |

### インタラクション

| メソッド | 説明 | Compose 対応 | SwiftUI 対応 |
|---------|------|-------------|-------------|
| `.on_press(msg)` | クリックハンドラ | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | リリースハンドラ | — | — |
| `.on_double_click(msg)` | ダブルクリック | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | 右クリック | — | `.contextMenu {}` |
| `.on_enter(msg)` | ホバー開始 | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | ホバー終了 | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | スクロールホイールイベント | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | マウス移動追跡 | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | カーソルスタイル | `.pointerInput()` | `.cursor()` |

### その他の機能

| メソッド | 説明 | Compose 対応 | SwiftUI 対応 |
|---------|------|-------------|-------------|
| `.tooltip_text()` | ツールチップ | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | ツールチップとコンテンツの間隔 | — | — |
| `.tooltip_padding()` | ツールチップ内側パディング | — | — |
| `.tooltip_snap()` | ビューポート内ツールチップスナップ | — | — |
| `.scrollable()` | 縦スクロール | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | 横スクロール | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | 双方向スクロール | — | — |
| `.scroll_anchor_bottom()` | 下端からスクロール開始 | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | 右端からスクロール開始 | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | スクロールバーとコンテンツの間隔 | — | — |
| `.scrollable_id()` | スクロールウィジェット ID | — | — |
| `.hidden(bool)` | 表示切替 | `AnimatedVisibility` | `.hidden()` |
| `.id()` | ウィジェット ID | `.testTag()` | `.id()` |
| `.font_size()` | フォントサイズ (ウィジェットラッパー) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | 子要素間隔 (ウィジェットラッパー) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### 合成と再利用

```rust
// 再利用可能なスタイル関数 (Modifier 使用)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// .modify() で適用 (後方互換)
text("Hello").modify(card_style())

// 条件付き modifier
Text::new("ステータス")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// 順序依存レイヤリング
Text::new("レイヤー")
    .padding(20)
    .layer()  // 現在のレイヤーをフラッシュ
    .background_color(Color::RED)  // padding の外側に適用
```

## Feature Flags

```toml
iced_modifier = { version = "0.5", features = ["icons", "drag-drop", "animation"] }
```

| Feature | クレート | 説明 |
|---------|---------|------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap アイコンフォント、`icon_label()` ヘルパー |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` 拡張 |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | `iced_anim` re-export |
| `image` | iced (image feature) | `Image` / `InteractiveImage` ウィジェットラッパー |
| `all` | 上記すべて | すべての機能を有効化 |

## サンプル実行

```bash
cargo run --example basic
cargo run --example showcase
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## マイグレーション

- [v0.4 → v0.5](migration/v0.4-to-v0.5.md) — TextAlign、Button ダイレクトスタイル
- [v0.1 → v0.2](migration/v0.1-to-v0.2.md) — ウィジェットラッパー、直接チェーン

## 互換性

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **プラットフォーム**: iced がサポートする全プラットフォーム (macOS, Windows, Linux, Web)

## ライセンス

MIT ライセンス。詳細は [LICENSE](../LICENSE) を参照してください。
