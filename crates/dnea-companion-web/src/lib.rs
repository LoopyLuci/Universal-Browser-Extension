//! Embedded companion UI.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The companion UI HTML.
pub const INDEX_HTML: &str = include_str!("../assets/index.html");
/// The companion UI JavaScript.
pub const APP_JS: &str = include_str!("../assets/app.js");
/// The companion UI stylesheet.
pub const STYLE_CSS: &str = include_str!("../assets/style.css");