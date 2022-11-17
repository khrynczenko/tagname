use tagname::TagName;

#[derive(TagName)]
pub union ArbitraryUnion {
    f1: u32,
    f2: f32,
}

fn main() {}
