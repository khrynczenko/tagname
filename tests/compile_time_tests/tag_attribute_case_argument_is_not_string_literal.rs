use tagname::TagName;

#[derive(TagName)]
enum Cases {
    #[tag(case = 1)]
    Lower,
    Upper
}

fn main() {
}
