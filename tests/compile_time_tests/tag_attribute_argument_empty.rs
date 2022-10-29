use tagname::TagName;

#[derive(TagName)]
enum Cases {
    #[tag]
    Lower,
    Upper
}

fn main() {
}
