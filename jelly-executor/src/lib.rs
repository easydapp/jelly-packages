#![doc = include_str!("../README.md")]
#![deny(unreachable_pub)] // ! lib needs to check this item
#![deny(unsafe_code)] // Deny the UNSAFE code
#![deny(missing_docs)] // ! Must write a document
#![warn(rustdoc::broken_intra_doc_links)] // Link validity in the document
#![warn(clippy::future_not_send)] // The object of asynchronous code association must be send
#![deny(clippy::unwrap_used)] // Deny unwrap
#![deny(clippy::expect_used)] // Deny expect
#![deny(clippy::panic)] // Deny panic

use std::cell::RefCell;

use context::CustomContext;
use error::ExecuteCodeError;

/// error
pub mod error;

/// context
pub mod context;

/// test
#[cfg(test)]
mod test;

// Common context
thread_local! {
    static CONTENT: RefCell<CustomContext> = RefCell::default();
}

/// execute code
///
/// # Arguments
///
/// * `code` - Code
/// * `args` - The string of parameter name and parameter. For example "[]" or "[[\"data\",\"{}\"]]"
pub fn execute_code(code: &str, args: &str) -> Result<String, ExecuteCodeError> {
    let args: Vec<(String, String)> =
        serde_json::from_str(args).map_err(|e| ExecuteCodeError::InvalidArgs(format!("{}", e)))?;
    let code = r##"
        inner = (#args#) => {
            let result = undefined;

            #code#

            return result;
        }

        result = inner(#values#);

        OpenJSON.stringify(result)
    "##
    .replace("#code#", code)
    .replace(
        "#args#",
        &args
            .iter()
            .map(|(name, _)| name.to_owned())
            .collect::<Vec<_>>()
            .join(", "),
    )
    .replace(
        "#values#",
        &args
            .iter()
            .map(|(_, value)| {
                if value.is_empty() {
                    "undefined".to_string()
                } else {
                    format!("OpenJSON.parse({:?})", value)
                }
            })
            .collect::<Vec<_>>()
            .join(", "),
    );

    // let mut context = CustomContext::default();
    // context.eval(&code)

    // Use thread itself context
    CONTENT.with_borrow_mut(|context| context.eval(&code))
}

/// execute validate code
///
/// # Arguments
///
/// * `code` - Code
/// * `value` - Verified value. For example "{\"text\":\"text\""}"
pub fn execute_validate_code(code: &str, value: &str) -> Result<String, ExecuteCodeError> {
    let code = r##"
        inner = (data) => {
            let result = undefined;

            #code#

            return result;
        }

        result = inner(#value#);

        OpenJSON.stringify(result)
    "##
    .replace("#code#", code)
    .replace(
        "#value#",
        &format!("OpenType.link_value_to_js_value(OpenJSON.parse({:?}))", value),
    );

    // let mut context = CustomContext::default();
    // context.eval(&code)

    // Use thread itself context
    CONTENT.with_borrow_mut(|context| context.eval(&code))
}
