//! Background color.

use super::token::roles;
use super::{Token, Tweak};

/// Set the background, or the widget's main fill (a switch's track).
pub fn bg(token: impl Into<Token>) -> Tweak {
    Tweak::Bg(token.into())
}

roles! {
    /// A theme role as a background: `Bg::Muted` is `bg(Token::Muted)`.
    Bg => Tweak::Bg
}
