#![deny(
    warnings,
    missing_debug_implementations,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    clippy::all,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
#![forbid(unsafe_code)]

pub use tagname_derive::TagName;

/// Exposes method `tag_name` to obtain a name (tag) of a currently hold
/// variant inside an `enum` instance.
pub trait TagName {
    fn tag_name(&self) -> &'static str;
}
