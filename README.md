tagname
=============

This library exports a trait called `TagName` that exposes a
`tag_name` method which is used for retrieving a name (tag) of a currently hold
variant within an `enum` value.

More importantly, together with `TagName` trait comes a `derive(TagName)`
macro that can automatically implement the trait.

```rust
use tagname::TagName;

#[derive(TagName)]
enum MyTaggedUnion {
    Yes,
    No,
    Maybe(usize),
}

#[test]
fn return_correct_tag_names() {
    let v1 = MyTaggedUnion::Yes;
    let v2 = MyTaggedUnion::No;
    let v3 = MyTaggedUnion::Maybe(1);
    assert_eq!(v1.tag_name(), "Yes");
    assert_eq!(v2.tag_name(), "No");
    assert_eq!(v3.tag_name(), "Maybe");
}
```
