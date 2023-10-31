use automated_test;

mod common;

#[test]
fn integration_test_it_adds_two() {
    common::setup();
    assert_eq!(4, automated_test::add_two(2));
}
