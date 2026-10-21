mod common;

/// Compiles `examples/arrays.syx` end-to-end.
#[test]
fn test_arrays_and_slices() -> color_eyre::Result<()> {
    common::compile_ok("examples/arrays.syx")?;
    Ok(())
}

/// Elements without a separating comma must not parse as an array
#[test]
fn test_array_literal_with_missing_comma_is_rejected() {
    common::compile_source_err_contains("func main(): void { let a = [1 2]; }", "','");
}

/// Elements without a separating comma must not parse as a vector
#[test]
fn test_vector_literal_with_missing_comma_is_rejected() {
    common::compile_source_err_contains("func main(): void { let v: []sint32 = {1 2}; }", "','");
}
