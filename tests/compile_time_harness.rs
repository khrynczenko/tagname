#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_time_tests/*.rs");
}
