#[test]
fn runner() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail_cases/*.rs");
}
