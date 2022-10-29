use tagname::TagName;

#[derive(TagName)]
enum Cases {
    #[tag(1)]
    Lower,
    Upper
}

fn main() {
}
