//! Helper utilities for working with [`predicates`](https://docs.rs/predicates),
//! inspired in part by
//! [Google Mock matchers](https://github.com/google/googletest/blob/master/googlemock/docs/cheat_sheet.md#matchers-matcherlist).

pub use predicates;
pub use predicates_tree;

/// Make predicate-based assertions with a better error message.
///
/// # Examples
///
/// ```
/// use assert_that::assert_that;
///
/// assert_that!("Hello World", str::similar("Hello World"));
///
/// assert_that!("Hello World", str::diff("Goodbye World"));
///
/// // Can be used with more complex predicates
/// assert_that!(
///     &1234,
///     ge(-5).and(le(i16::MAX)),
/// );
/// ```
///
/// Note that `predicate::*` functions from `predicates::prelude` are brought into
/// scope automatically when used within the macro invocation. See
/// `predicates` [documentation](https://docs.rs/predicates) for details about
/// available predicates.
#[macro_export]
macro_rules! assert_that {
    ($value:expr, $pred:expr $(,)?) => {{
        use $crate::predicates::prelude::*;
        use $crate::predicates_tree::CaseTreeExt;

        use predicate::*;

        if let Some(case) = $pred.find_case(false, $value) {
            panic!("{}", case.tree());
        };
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_success() {
        assert_that!(&(), always());
        assert_that!(&(), never().not());
        assert_that!(&(2 + 2), eq(4));
    }

    #[test]
    fn test_accept_trailing_comma() {
        assert_that!(
            "Always have a needle in your haystack",
            str::contains("needle"),
        );
    }

    #[test]
    #[should_panic(expected = "(var == 5 && var > 3)\n└── var == 5\n")]
    fn test_failure_shows_tree() {
        assert_that!(&(2 + 2), eq(5).and(gt(3)))
    }
}
