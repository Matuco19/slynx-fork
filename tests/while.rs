mod common;

/// Compiles `examples/while.syx` and asserts the generated artifact is a `.sir`
/// output.
#[test]
fn test_while() -> color_eyre::Result<()> {
    common::compile_ok("examples/while.syx")?;
    Ok(())
}
