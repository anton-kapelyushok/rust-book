use adder;

mod common;

#[test]
fn use_module() {
    common::setup();
}

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}