mod common;

#[test]
fn simple_test() {
    common::setup();
    assert_eq!(3 + 2, 5);
}
