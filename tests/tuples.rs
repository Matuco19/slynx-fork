mod common;

/// Compiles `examples/tupleAccess.syx` (a regression fixture: the tuple's
/// first field is a concrete Struct TypeId, not a Reference), asserting the
/// generated artifact is a `.sir` output.
#[test]
fn test_tuple_access() {
    common::compile_ok_sir("examples/tupleAccess.syx");
}

/// Compiles `examples/tupleTwoObjects.syx`, asserting the generated artifact is
/// a `.sir` output.
#[test]
fn test_tuple_two_objects() {
    common::compile_ok_sir("examples/tupleTwoObjects.syx");
}

/// Compiles `examples/tupleNestedObject.syx`, asserting the generated artifact
/// is a `.sir` output.
#[test]
fn test_tuple_nested_object() {
    common::compile_ok_sir("examples/tupleNestedObject.syx");
}

/// Regression for #167: the empty tuple `()` is the unit type and literal
#[test]
fn test_empty_tuple_type_and_literal() {
    common::compile_source_ok("func g(): () { () } func main(): void { g(); }");
}

/// Regression for #167: `()` is valid as a parameter type and call argument
#[test]
fn test_empty_tuple_as_parameter() {
    common::compile_source_ok("func f(x: ()): void { () } func main(): void { f(()); }");
}

/// Tuple annotations lower to tuple types instead of a missing named type
#[test]
fn test_tuple_type_annotation() {
    common::compile_source_ok(
        "func pair(): (int, str) { (1, \"ok\") } func main(): void { pair(); }",
    );
}

/// Elements without a separating comma must not parse as a tuple
#[test]
fn test_tuple_literal_with_missing_comma_is_rejected() {
    common::compile_source_err_contains("func main(): void { let t = (1, 2 3); }", "','");
}

/// The empty tuple exposes 0 fields, so any positional access is a type error
#[test]
fn test_empty_tuple_access_is_rejected() {
    let err = common::compile_source_err_contains(
        "func main(): int { let unit = (); unit.0 }",
        "Tuple index",
    );
    assert!(
        err.contains("out of bounds"),
        "expected out-of-bounds, got: {err}"
    );
}
