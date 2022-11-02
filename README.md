tagname
=============
[<img alt="github" src="https://img.shields.io/badge/github-khrynczenko/tagname-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/khrynczenko/tagname)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tagname.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tagname)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tagname-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tagname)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/khrynczenko/tagname/Rust/master?style=for-the-badge" height="20">](https://github.com/khrynczenko/tagname/actions?query=branch%3Amaster)

This library exports a trait called `TagName` that exposes a
`tag_name` method which is used for retrieving a name (tag) of a currently hold variant within an `enum` value.

More importantly, together with `TagName` trait comes a `derive(TagName)`
macro that can automatically implement the trait.

```rust
use tagname::TagName;

#[derive(TagName)]
enum MyTaggedUnion {
    [tag(case = "lower")]
    Yes,
    [tag(case = "upper")]
    No,
    Maybe(usize),
}

#[test]
fn return_correct_tag_names() {
    let v1 = MyTaggedUnion::Yes;
    let v2 = MyTaggedUnion::No;
    let v3 = MyTaggedUnion::Maybe(1);
    assert_eq!(v1.tag_name(), "yes");
    assert_eq!(v2.tag_name(), "NO");
    assert_eq!(v3.tag_name(), "Maybe");
}
```
