pub use tagname_derive::Variant;

pub trait Variant {
    fn tag_name(&self) -> &'static str;
}
