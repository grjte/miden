use std::cmp::Ordering;

use super::{
    build_op_test, test_inputs_out_of_bounds, test_param_out_of_bounds, test_unsafe_execution,
    TestError, U32_BOUND,
};
use proptest::prelude::*;
use rand_utils::rand_value;

// U32 OPERATIONS TESTS - MANUAL - COMPARISON OPERATIONS
// ================================================================================================

#[test]
fn u32eq() {
    let asm_op = "u32eq";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[0]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64, a as u64]);
    test.expect_stack(&[1]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a == b { 1 } else { 0 };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(asm_op, &[c, a as u64, b as u64]);
    test.expect_stack(&[expected, c]);
}

#[test]
fn u32eq_fail() {
    let asm_op = "u32eq";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32eq_b() {
    let build_asm_op = |param: u32| format!("u32eq.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(build_asm_op(1).as_str(), &[1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(build_asm_op(0).as_str(), &[1]);
    test.expect_stack(&[0]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;

    let test = build_op_test!(build_asm_op(a).as_str(), &[a as u64]);
    test.expect_stack(&[1]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a == b { 1 } else { 0 };

    let test = build_op_test!(build_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(build_asm_op(b).as_str(), &[c, a as u64]);
    test.expect_stack(&[expected, c]);
}

#[test]
fn u32eq_b_fail() {
    let asm_op = "u32eq";

    // should fail when b is out of bounds and provided as a parameter
    test_param_out_of_bounds(asm_op, U32_BOUND);

    // should fail when b is a valid parameter but a is out of bounds
    let asm_op = format!("{}.{}", asm_op, 1);
    let test = build_op_test!(&asm_op, &[U32_BOUND]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));
}

#[test]
fn u32neq() {
    let asm_op = "u32neq";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[0]);

    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[1]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64, a as u64]);
    test.expect_stack(&[0]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a != b { 1 } else { 0 };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(asm_op, &[c, a as u64, b as u64]);
    test.expect_stack(&[expected, c]);
}

#[test]
fn u32neq_fail() {
    let asm_op = "u32neq";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32neq_b() {
    let build_asm_op = |param: u32| format!("u32neq.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(build_asm_op(1).as_str(), &[1]);
    test.expect_stack(&[0]);

    let test = build_op_test!(build_asm_op(0).as_str(), &[1]);
    test.expect_stack(&[1]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;

    let test = build_op_test!(build_asm_op(a).as_str(), &[a as u64]);
    test.expect_stack(&[0]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a != b { 1 } else { 0 };

    let test = build_op_test!(build_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(build_asm_op(b).as_str(), &[c, a as u64]);
    test.expect_stack(&[expected, c]);
}

#[test]
fn u32neq_b_fail() {
    let asm_op = "u32neq";

    // should fail when b is out of bounds and provided as a parameter
    test_param_out_of_bounds(asm_op, U32_BOUND);

    // should fail when b is a valid parameter but a is out of bounds
    let asm_op = format!("{}.{}", asm_op, 1);
    let test = build_op_test!(&asm_op, &[U32_BOUND]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));
}

#[test]
fn u32lt() {
    let asm_op = "u32lt";

    // should push 1 to the stack when a < b and 0 otherwise
    test_comparison_op(asm_op, 1, 0, 0);
}

#[test]
fn u32lt_fail() {
    let asm_op = "u32lt";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32lt_unsafe() {
    let asm_op = "u32lt.unsafe";

    // should push 1 to the stack when a < b and 0 otherwise
    test_comparison_op(asm_op, 1, 0, 0);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32lte() {
    let asm_op = "u32lte";

    // should push 1 to the stack when a <= b and 0 otherwise
    test_comparison_op(asm_op, 1, 1, 0);
}

#[test]
fn u32lte_fail() {
    let asm_op = "u32lte";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32lte_unsafe() {
    let asm_op = "u32lte.unsafe";

    // should push 1 to the stack when a <= b and 0 otherwise
    test_comparison_op(asm_op, 1, 1, 0);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32gt() {
    let asm_op = "u32gt";

    // should push 1 to the stack when a > b and 0 otherwise
    test_comparison_op(asm_op, 0, 0, 1);
}

#[test]
fn u32gt_fail() {
    let asm_op = "u32gt";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32gt_unsafe() {
    let asm_op = "u32gt.unsafe";

    // should push 1 to the stack when a > b and 0 otherwise
    test_comparison_op(asm_op, 0, 0, 1);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32gte() {
    let asm_op = "u32gte";

    // should push 1 to the stack when a >= b and 0 otherwise
    test_comparison_op(asm_op, 0, 1, 1);
}

#[test]
fn u32gte_fail() {
    let asm_op = "u32gte";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32gte_unsafe() {
    let asm_op = "u32gte.unsafe";

    // should push 1 to the stack when a >= b and 0 otherwise
    test_comparison_op(asm_op, 0, 1, 1);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32min() {
    let asm_op = "u32min";

    // should put the minimum of the 2 inputs on the stack
    test_min(asm_op);
}

#[test]
fn u32min_fail() {
    let asm_op = "u32min";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32min_unsafe() {
    let asm_op = "u32min.unsafe";

    // should put the minimum of the 2 inputs on the stack
    test_min(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32max() {
    let asm_op = "u32max";

    // should put the maximum of the 2 inputs on the stack
    test_max(asm_op);
}

#[test]
fn u32max_fail() {
    let asm_op = "u32max";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32max_unsafe() {
    let asm_op = "u32max.unsafe";

    // should put the maximum of the 2 inputs on the stack
    test_max(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

// U32 OPERATIONS TESTS - RANDOMIZED - COMPARISON OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    fn u32eq_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32eq";
        let values = [b as u64, a as u64];

        // should test for equality
        let expected = if a == b { 1 } else { 0 };
        // b provided via the stack
        let test = build_op_test!(asm_op, &values);
        test.prop_expect_stack(&[expected])?;

        // b provided as a parameter
        let asm_op = format!("{}.{}", asm_op, b);
        let test = build_op_test!(&asm_op, &[a as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32neq_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32neq";
        let values = [b as u64, a as u64];

        // should test for inequality
        let expected = if a != b { 1 } else { 0 };
        // b provided via the stack
        let test = build_op_test!(asm_op, &values);
        test.prop_expect_stack(&[expected])?;

        // b provided as a parameter
        let asm_op = format!("{}.{}", asm_op, b);
        let test = build_op_test!(&asm_op, &[a as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32lt_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32lt";
        let expected = match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 0,
        };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32lte_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32lte";
        let expected = match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 1,
            Ordering::Greater => 0,
        };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32gt_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32gt";
        let expected = match a.cmp(&b) {
            Ordering::Less => 0,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32gte_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32gte";
        let expected = match a.cmp(&b) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 1,
        };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32min_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32min";
        let expected = if a < b { a } else { b };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }

    #[test]
    fn u32max_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32max";
        let expected = if a > b { a } else { b };

        // safe and unsafe should produce the same result for valid values
        let test = build_op_test!(asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;

        let asm_op = format!("{}.unsafe", asm_op);
        let test = build_op_test!(&asm_op, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// This helper function tests that the provided assembly comparison operation pushes the expected
/// value to the stack for each of the less than, equal to, or greater than comparisons tested.
fn test_comparison_op(asm_op: &str, expected_lt: u64, expected_eq: u64, expected_gt: u64) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put the expected value on the stack for the less-than case
    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[expected_lt]);

    // a = b should put the expected value on the stack for the equal-to case
    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[expected_eq]);

    // a > b should put the expected value on the stack for the greater-than case
    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[expected_gt]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => expected_lt,
        Ordering::Equal => expected_eq,
        Ordering::Greater => expected_gt,
    };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(asm_op, &[c, a as u64, b as u64]);
    test.expect_stack(&[expected, c]);
}

/// Tests a u32min assembly operation (u32min or u32min.unsafe) against a number of cases to ensure
/// that the operation puts the minimum of 2 input values on the stack.
fn test_min(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put a on the stack
    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[0]);

    // a = b should put b on the stack
    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[0]);

    // a > b should put b on the stack
    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Equal => b,
        Ordering::Greater => b,
    };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(asm_op, &[c, a as u64, b as u64]);
    test.expect_stack(&[expected as u64, c]);
}

/// Tests a u32max assembly operation (u32max or u32max.unsafe) against a number of cases to ensure
/// that the operation puts the maximum of 2 input values on the stack.
fn test_max(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put b on the stack
    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[1]);

    // a = b should put b on the stack
    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[0]);

    // a > b should put a on the stack
    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => b,
        Ordering::Equal => b,
        Ordering::Greater => a,
    };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();

    let test = build_op_test!(asm_op, &[c, a as u64, b as u64]);
    test.expect_stack(&[expected as u64, c]);
}
