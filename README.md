# assert_that

Helper macro for use with [`predicates`](https://crates.io/crates/predicates).
This macro prints more detailed output on failure compared to `assert!`, using
output from [`predicates-tree`](https://crates.io/crates/predicates-tree).

## Usage

### Example

```rust
extern crate assert_that;
use assert_that::assert_that;

assert_that!("Hello World", str::similar("Hello World"));
assert_that!(3, eq(2 + 1));
```

Note that `predicate::*` functions from `predicates::prelude` are brought into
scope automatically when used within the macro invocation. See
`predicates` [documentation](https://docs.rs/predicates) for more details.

See [this crate's documentation](https://docs.rs/assert_that) for more detailed examples.
