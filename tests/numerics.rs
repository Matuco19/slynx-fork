mod common;

/// Discovers and compiles every `.slx` example in `examples/numerics/`.
/// Each file is a focused test case for integer widths, their limits and the
/// values that do or do not fit them.
/// Files marked with `// xfail:` are expected to be rejected (or to panic)
/// today; files marked with `// xpass:` are expected to compile even though they
/// should eventually be rejected. Any change in their outcome is reported so the
/// marker can be updated.
#[test]
fn numerics_examples() {
    common::run_examples("examples/numerics", "numerics");
}
