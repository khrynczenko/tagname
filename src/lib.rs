pub use tagname_derive::TagName;

pub trait TagName {
    fn tag_name(&self) -> &'static str;
}
