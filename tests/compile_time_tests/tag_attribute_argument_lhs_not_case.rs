use tagname::TagName;

#[derive(TagName)]
enum Cases {
    #[tag(something = "something")]
    Lower,
    Upper
}

fn main() {
}
