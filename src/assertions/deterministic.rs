use regex::Regex;
use serde_json::{json, Value};

use super::{Assertion, AssertionContext, AssertionResult};

pub fn evaluate_deterministic(
    assertion: &Assertion,
    context: &AssertionContext,
) -> AssertionResult {
    match assertion {
        Assertion::Equals(expected) => evaluate_equals(expected, context),
        Assertion::Contains(expected) => evaluate_contains(expected, context),
        Assertion::Regex(pattern) => evaluate_regex(pattern, context),
        Assertion::JsonPointer { pointer, expected } => {
            evaluate_json_pointer(pointer, expected, context)
        }
        Assertion::JsonSchema(schema) => evaluate_json_schema(schema, context),
    }
}

fn evaluate_equals(expected: &Value, context: &AssertionContext) -> AssertionResult {
    if &context.output == expected {
        AssertionResult::passed("equals")
    } else {
        AssertionResult::failed(
            "equals",
            format!(
                "expected output to equal {expected}, got {}",
                context.output
            ),
        )
        .with_metadata(json!({ "expected": expected, "actual": context.output }))
    }
}

fn evaluate_contains(expected: &str, context: &AssertionContext) -> AssertionResult {
    if context.output_text.contains(expected) {
        AssertionResult::passed("contains")
    } else {
        AssertionResult::failed("contains", format!("expected output to contain {expected}"))
    }
}

fn evaluate_regex(pattern: &str, context: &AssertionContext) -> AssertionResult {
    let regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => return AssertionResult::error("regex", error.to_string()),
    };

    if regex.is_match(&context.output_text) {
        AssertionResult::passed("regex")
    } else {
        AssertionResult::failed("regex", format!("expected output to match {pattern}"))
    }
}

fn evaluate_json_pointer(
    pointer: &str,
    expected: &Value,
    context: &AssertionContext,
) -> AssertionResult {
    match context.output.pointer(pointer) {
        Some(actual) if actual == expected => AssertionResult::passed("json"),
        Some(actual) => AssertionResult::failed(
            "json",
            format!("expected {pointer} to equal {expected}, got {actual}"),
        )
        .with_metadata(json!({ "pointer": pointer, "expected": expected, "actual": actual })),
        None => AssertionResult::failed("json", format!("missing JSON pointer {pointer}")),
    }
}

fn evaluate_json_schema(schema: &Value, context: &AssertionContext) -> AssertionResult {
    let compiled = match jsonschema::JSONSchema::compile(schema) {
        Ok(compiled) => compiled,
        Err(error) => return AssertionResult::error("schema", error.to_string()),
    };

    let result = match compiled.validate(&context.output) {
        Ok(()) => AssertionResult::passed("schema"),
        Err(errors) => {
            let messages = errors.map(|error| error.to_string()).collect::<Vec<_>>();
            AssertionResult::failed("schema", messages.join("; "))
        }
    };
    result
}
