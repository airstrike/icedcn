//! Foreground color.

use super::token::roles;
use super::{Token, Tweak};

/// Set the foreground: text on typography widgets, the bar of a progress
/// bar, a checkbox's icon, a switch's thumb.
pub fn fg(token: impl Into<Token>) -> Tweak {
    Tweak::Fg(token.into())
}

roles! {
    /// A theme role as a foreground: `Fg::MutedForeground` is
    /// `fg(Token::MutedForeground)`.
    Fg => Tweak::Fg
}
