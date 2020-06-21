extern crate assert_that;

use assert_that::assert_that;

fn main() {
    assert_that!("Hello World", str::similar("Hello World"));

    assert!(always());
}
