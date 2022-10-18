use tagname::TagName;

#[derive(TagName)]
enum MyVariant {
    Yes,
    No,
    Maybe(usize),
}

#[test]
fn return_correct_tag_names() {
    let v1 = MyVariant::Yes;
    let v2 = MyVariant::No;
    let v3 = MyVariant::Maybe(1);
    assert_eq!(v1.tag_name(), "Yes");
    assert_eq!(v2.tag_name(), "No");
    assert_eq!(v3.tag_name(), "Maybe");
}
