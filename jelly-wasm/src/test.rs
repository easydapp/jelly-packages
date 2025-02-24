use wasm_bindgen_test::console_error as test_console_error;
use wasm_bindgen_test::console_log as test_console_log;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test() {
    let result1 = jelly_executor::execute_code(r#"result = 1 + 2;"#, "[]");
    match result1 {
        Ok(r) => test_console_log!("{}", r),
        Err(e) => test_console_error!("{:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use jelly_executor::error::ExecuteCodeError;

    #[test]
    fn test() {
        println!("Error: {:?}", ExecuteCodeError::InvalidArgs("123".into()));
        println!("Error: {:?}", ExecuteCodeError::InvalidOutput("123".into()));
        println!("Error: {:?}", ExecuteCodeError::Undefined);
        println!("Error: {:?}", ExecuteCodeError::WrongOutput("123".into()));
        println!("Error: {:?}", ExecuteCodeError::ExecuteError("123".into()));
    }

    #[test]
    fn test2() {
        let result1 = jelly_executor::execute_code(
            r#"const fi = (n) => {
            if (n === 1 || n === 2) return 1;
            return fi(n - 1) + fi(n - 2);
        }
        result = fi(31);"#,
            "[]",
        );
        println!("Result: {:?}", result1);
    }
}
