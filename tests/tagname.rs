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

#[allow(dead_code)]
struct Arbitrary {
    x: String,
    y: Option<usize>,
}

#[derive(TagName)]
enum ComplexUnion {
    Yes,
    No,
    Maybe(usize),
    Maybe2(usize, usize),
    Maybe3(Arbitrary),
    Maybe4 { x: Option<String> },
}

#[derive(TagName)]
enum ComplexUnionSingleTag {
    Maybe(usize),
}

#[derive(TagName)]
enum DifferentCasesUnion {
    #[tag(case = "lower")]
    Lower,
    #[tag(case = "upper")]
    Upper,
    #[tag(case = "lower")]
    LowerWith(usize),
    #[tag(case = "upper")]
    UpperWith(usize),
    Normal(usize),
}

// It happened that I ommited the fact that other attributes can be also used
// on the same field and I need to take this into account so this checks that
// compilation still succeeds.
#[allow(dead_code)]
#[allow(invalid_doc_attributes)]
#[derive(TagName)]
enum CheckTagNameAllowsOtherAttributes {
    #[tag(case = "lower")]
    Tag1(usize),
    #[doc("other attribute")]
    #[tag(case = "upper")]
    Tag2(usize),
    #[tag(case = "upper")]
    #[doc("other attribute")]
    Tag3(usize),
    #[doc("other attribute")]
    #[tag(case = "upper")]
    #[doc("other attribute")]
    Tag4(usize),
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
    let v5 = ComplexUnion::Maybe3(Arbitrary {
        x: String::from("x"),
        y: Some(1),
    });
    let v6 = ComplexUnion::Maybe4 {
        x: Some("x".to_owned()),
    };
    assert_eq!(v1.tag_name(), "Yes");
    assert_eq!(v2.tag_name(), "No");
    assert_eq!(v3.tag_name(), "Maybe");
    assert_eq!(v4.tag_name(), "Maybe2");
    assert_eq!(v5.tag_name(), "Maybe3");
    assert_eq!(v6.tag_name(), "Maybe4");
}

#[test]
fn complex_union_single_tag_return_correct_tag_names() {
    let v3 = ComplexUnionSingleTag::Maybe(1);
    assert_eq!(v3.tag_name(), "Maybe");
}

#[test]
fn diffrent_cases_are_applied() {
    let v1 = DifferentCasesUnion::Lower;
    let v2 = DifferentCasesUnion::Upper;
    let v3 = DifferentCasesUnion::LowerWith(1);
    let v4 = DifferentCasesUnion::UpperWith(1);
    let v5 = DifferentCasesUnion::Normal(1);

    assert_eq!(v1.tag_name(), "lower");
    assert_eq!(v2.tag_name(), "UPPER");
    assert_eq!(v3.tag_name(), "lowerwith");
    assert_eq!(v4.tag_name(), "UPPERWITH");
    assert_eq!(v5.tag_name(), "Normal");
}
