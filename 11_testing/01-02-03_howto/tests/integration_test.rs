// Here you test the API and modules together in general, more high-level

use howto;

mod common;

#[test]
fn api_tests_it_adds_two() {
    common::setup();
    assert_eq!(4, howto::add_two(2));
}
