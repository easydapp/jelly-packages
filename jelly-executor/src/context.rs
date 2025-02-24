use boa_engine::{Context, Source};

use crate::error::ExecuteCodeError;

/// Introduce JS code
const BUNDLE_JS: &str = include_str!("../../jelly-types/dist/bundle.js");

/// Customized context
pub struct CustomContext {
    context: Context,
}

impl Default for CustomContext {
    fn default() -> Self {
        let mut context = Context::default();

        let code = r##"
            const {
                OpenJSON,
                OpenType,
                OpenNumber,
                OpenHex,

                Principal,
                OpenIc
            } = (() => {
                let self = {};
                #bundle.js#
                return self.default;
            })();
            OpenJSON.parse = OpenJSON.parse_factory(JSON.parse);
            OpenJSON.stringify = OpenJSON.stringify_factory(JSON.stringify);

            let inner;
            let result;
        "##
        .replace("#bundle.js#", BUNDLE_JS);

        #[allow(clippy::unwrap_used)] // ? SAFETY
        context.eval(Source::from_bytes(&code)).unwrap();

        Self { context }
    }
}

impl CustomContext {
    /// Execute code
    pub fn eval(&mut self, code: &str) -> Result<String, ExecuteCodeError> {
        let source = Source::from_bytes(code);
        let value = self.context.eval(source);

        // Uniformly process the return data, the execution result itself must be a string
        let result = match value {
            Ok(boa_engine::value::JsValue::String(v)) => match v.to_std_string() {
                Ok(v) => Ok(v),
                Err(e) => Err(ExecuteCodeError::InvalidOutput(format!("Encode {}: {:?}", e, v))),
            },
            Ok(boa_engine::value::JsValue::Undefined) => Err(ExecuteCodeError::Undefined),
            Ok(v) => Err(ExecuteCodeError::WrongOutput(format!("Not JsString Error: {:?}", v))),
            Err(e) => Err(ExecuteCodeError::ExecuteError(format!("{}", e))),
        };
        if result.is_err() {
            println!("code:");
            println!("{}", code);
        }

        result
    }
}
