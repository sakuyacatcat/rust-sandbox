extern crate rust_test;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_test::add_two(2));
}
