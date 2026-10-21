mod common;

use common::{compile_source_err_contains, compile_source_ok};

/// Regression test for #171. unary `-` must work on integer and float literals as well as on variables of those types
#[test]
fn negates_numeric_literals_and_variables() {
    compile_source_ok(
        "func negate_int(value: int): int -> -value;
         func negate_float(value: f32): f32 -> -value;
         func main(): int {
             let literal: int = -5;
             let float_literal: f32 = -1.5;
             let negated: int = negate_int(literal);
             let negated_float: f32 = negate_float(float_literal);
             negated
         }",
    );
}

/// `-` is only defined for signed integers and floats
#[test]
fn rejects_negation_of_a_non_numeric_value() {
    compile_source_err_contains("func main(): int { -true }", "unary '-'");
}
