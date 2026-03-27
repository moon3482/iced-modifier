/// Icon font utilities (requires `icons` feature).
///
/// Re-exports `iced_fonts` icon modules and provides convenience helpers.
///
/// # Example
/// ```ignore
/// use iced_modifier::icons::{bootstrap, BOOTSTRAP_FONT_BYTES};
///
/// // In your app's font loader:
/// fn boot() -> (App, Task<Message>) {
///     (App::default(), load_icon_fonts())
/// }
///
/// // Use icons directly as Text widgets:
/// bootstrap::check()   // returns Text with check icon
/// bootstrap::gear()    // returns Text with gear icon
/// ```
pub use iced_fonts::bootstrap;
pub use iced_fonts::{BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES};

use iced::widget::{row, text, Row};
use iced::{Element, Font};

/// Load all enabled icon fonts. Call this in your app's boot/init function.
///
/// Returns a `Task` that loads the Bootstrap icon font.
/// The `Result` indicates whether the font loaded successfully.
pub fn load_icon_fonts() -> iced::Task<Result<(), iced::font::Error>> {
    iced::font::load(BOOTSTRAP_FONT_BYTES)
}

/// Create a row with an icon and label text, spaced apart.
///
/// # Example
/// ```ignore
/// use iced_modifier::icons::{bootstrap, icon_label};
///
/// icon_label(bootstrap::check(), "Complete")
/// ```
pub fn icon_label<'a, Message, Theme, Renderer>(
    icon: impl Into<Element<'a, Message, Theme, Renderer>>,
    label: &str,
) -> Row<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font> + 'a,
{
    row![icon.into(), text(label.to_string())].spacing(8)
}
