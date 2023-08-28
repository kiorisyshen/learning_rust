use chp10_type_lifetime;

mod common;

// cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, chp10_type_lifetime::add_two(2));
}
