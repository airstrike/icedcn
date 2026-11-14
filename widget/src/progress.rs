//! A four-pixel progress bar with a muted track and a primary-colored fill. When the value
//! changes, the fill animates from its current position to the new target over 150 ms.
//!
//! # Example
//!
//! ```ignore
//! use iced::Fill;
//! use icedcn::widget::progress;
//!
//! progress(value).width(Fill)
//! ```

mod element;
pub mod widget;

pub use element::{Progress, progress};
