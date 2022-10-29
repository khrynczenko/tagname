use tagname::TagName;

#[derive(TagName)]
enum SimpleUnion {
    Yes,
    No,
}

#[derive(TagName)]
enum SimpleUnionSingleTag {
    Yes,
}

#[derive(TagName)]
enum ComplexUnion {
    Yes,
    No,
    Maybe(usize),
    Maybe2(usize, usize),
}

#[derive(TagName)]
enum ComplexUnionSingleTag {
    Maybe(usize),
}

#[test]
fn simple_union_return_correct_tag_names() {
    let v1 = SimpleUnion::Yes;
    let v2 = SimpleUnion::No;
    assert_eq!(v1.tag_name(), "Yes");
    assert_eq!(v2.tag_name(), "No");
}

#[test]
fn simple_union_single_tag_return_correct_tag_names() {
    let v1 = SimpleUnionSingleTag::Yes;
    assert_eq!(v1.tag_name(), "Yes");
}

#[test]
fn complex_union_return_correct_tag_names() {
    let v1 = ComplexUnion::Yes;
    let v2 = ComplexUnion::No;
    let v3 = ComplexUnion::Maybe(1);
    let v4 = ComplexUnion::Maybe2(1, 2);
    assert_eq!(v1.tag_name(), "Yes");
    assert_eq!(v2.tag_name(), "No");
    assert_eq!(v3.tag_name(), "Maybe");
    assert_eq!(v4.tag_name(), "Maybe2");
}

#[test]
fn complex_union_single_tag_return_correct_tag_names() {
    let v3 = ComplexUnionSingleTag::Maybe(1);
    assert_eq!(v3.tag_name(), "Maybe");
}
