use tagname::TagName;

#[derive(TagName)]
enum Cases {
    #[tag(case = "something")]
    Lower,
    Upper
}

fn main() {
}
