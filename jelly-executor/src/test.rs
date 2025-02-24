use crate::{execute_code, execute_validate_code};

#[cfg(test)]
mod tests {
    use crate::error::ExecuteCodeError;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            execute_validate_code(
                r#"result = data.length === 5 ? '' : 'length must be 5';"#,
                r#"{"text":"12345"}"#
            ),
            Ok(r#""""#.to_string())
        );
        assert_eq!(
            execute_validate_code(
                r#"result = data.length === 5 ? '' : 'length must be 5';"#,
                r#"{"text":"123"}"#
            ),
            Ok(r#""length must be 5""#.to_string())
        );
        assert_eq!(execute_code(r#"result = 1 + 2;"#, "[]"), Ok(r#"3"#.to_string()));

        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(undefined);"#, "[]"),
            Err(ExecuteCodeError::Undefined)
        );

        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(null);"#, "[]"),
            Ok(r#""null""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify("text");"#, "[]"),
            Ok(r#""\"text\"""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(123);"#, "[]"),
            Ok(r#""123""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(123.123);"#, "[]"),
            Ok(r#""123.123""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(123.123e-3);"#, "[]"),
            Ok(r#""0.123123""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(true);"#, "[]"),
            Ok(r#""true""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(false);"#, "[]"),
            Ok(r#""false""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(1n);"#, "[]"),
            Ok(r#""{\"__open_type__\":\"bigint\",\"value\":\"1\"}""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify((() => {}));"#, "[]"),
            Err(ExecuteCodeError::ExecuteError(
                "Error: can not stringify function".into()
            ))
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify([1,2,3]);"#, "[]"),
            Ok(r#""[1,2,3]""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify([1,"2",true,4n]);"#, "[]"),
            Ok(r#""[1,\"2\",true,{\"__open_type__\":\"bigint\",\"value\":\"4\"}]""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify(new Uint8Array([1,2,3]));"#, "[]"),
            Ok(r#""{\"__open_type__\":\"Uint8Array\",\"value\":[1,2,3]}""#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = Principal.fromText("udtw4-baaaa-aaaah-abc3q-cai").toText();"#,
                "[]"
            ),
            Ok(r#""udtw4-baaaa-aaaah-abc3q-cai""#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.stringify(Principal.fromText("udtw4-baaaa-aaaah-abc3q-cai"));"#,
                "[]"
            ),
            Ok(r#""{\"__open_type__\":\"Principal\",\"value\":\"udtw4-baaaa-aaaah-abc3q-cai\"}""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.stringify({a:1,b:true,c:3n,d:[1,2,3]});"#, "[]"),
            Ok(r#""{\"a\":1,\"b\":true,\"c\":{\"__open_type__\":\"bigint\",\"value\":\"3\"},\"d\":[1,2,3]}""#.into())
        );

        assert_eq!(
            execute_code(r#"result = OpenJSON.parse(null);"#, "[]"),
            Err(ExecuteCodeError::ExecuteError(
                "Error: json must be a string".to_string()
            ))
        );

        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("null");"#, "[]"),
            Ok(r#"null"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("\"text\"");"#, "[]"),
            Ok(r#""text""#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("123");"#, "[]"),
            Ok(r#"123"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("123.123");"#, "[]"),
            Ok(r#"123.123"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("123.123e-3");"#, "[]"),
            Ok(r#"0.123123"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("true");"#, "[]"),
            Ok(r#"true"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("false");"#, "[]"),
            Ok(r#"false"#.to_string())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.parse("{\"__open_type__\":\"bigint\",\"value\":\"1\"}");"#,
                "[]"
            ),
            Ok(r#"{"__open_type__":"bigint","value":"1"}"#.to_string())
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse((() => {}));"#, "[]"),
            Err(ExecuteCodeError::ExecuteError("Error: json must be a string".into()))
        );
        assert_eq!(
            execute_code(r#"result = OpenJSON.parse("[1,2,3]");"#, "[]"),
            Ok(r#"[1,2,3]"#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.parse("[1,\"2\",true,{\"__open_type__\":\"bigint\",\"value\":\"4\"}]");"#,
                "[]"
            ),
            Ok(r#"[1,"2",true,{"__open_type__":"bigint","value":"4"}]"#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.parse("{\"__open_type__\":\"Uint8Array\",\"value\":[1,2,3]}");"#,
                "[]"
            ),
            Ok(r#"{"__open_type__":"Uint8Array","value":[1,2,3]}"#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.parse("{\"__open_type__\":\"Principal\",\"value\":\"udtw4-baaaa-aaaah-abc3q-cai\"}");"#,
                "[]"
            ),
            Ok(r#"{"__open_type__":"Principal","value":"udtw4-baaaa-aaaah-abc3q-cai"}"#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenJSON.parse("{\"a\":1,\"b\":true,\"c\":{\"__open_type__\":\"bigint\",\"value\":\"3\"},\"d\":[1,2,3]}");"#,
                "[]"
            ),
            Ok(r#"{"a":1,"b":true,"c":{"__open_type__":"bigint","value":"3"},"d":[1,2,3]}"#.into())
        );

        assert_eq!(
            execute_code(r#"result = OpenType.link_value_to_js_value({text: "text"});"#, "[]"),
            Ok(r#""text""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenType.link_value_to_js_value({integer: 123});"#, "[]"),
            Ok(r#"123"#.into())
        );

        assert_eq!(
            execute_code(r#"result = OpenNumber.format_integer("123123123");"#, "[]"),
            Ok(r#""123,123,123""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenNumber.format_number("123123123.12312312");"#, "[]"),
            Ok(r#""123,123,123.123,123,12""#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenNumber.unit(8);"#, "[]"),
            Ok(r#""100000000""#.into())
        );

        assert_eq!(
            execute_code(r#"result = OpenHex.hex2array("0x0102");"#, "[]"),
            Ok(r#"[1,2]"#.into())
        );
        assert_eq!(
            execute_code(r#"result = OpenHex.array2hex([1,2]);"#, "[]"),
            Ok(r#""0102""#.into())
        );

        assert_eq!(
            execute_code(r#"result = Principal.fromText("udtw4-baaaa-aaaah-abc3q-cai");"#, "[]"),
            Ok(r#"{"__open_type__":"Principal","value":"udtw4-baaaa-aaaah-abc3q-cai"}"#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = data.toText();"#,
                r#"[["data", "{\"__open_type__\":\"Principal\",\"value\":\"chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe\"}"]]"#
            ),
            Ok(r#""chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe""#.into())
        );

        assert_eq!(
            execute_code(
                r#"result = OpenIc.principal2account_id(data.toText());"#,
                r#"[["data", "{\"__open_type__\":\"Principal\",\"value\":\"chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe\"}"]]"#
            ),
            Ok(r#""ec267d2835aef4dc6b69007ad51f49efcd0850324c059ea314cc81849480beb4""#.into())
        );
        assert_eq!(
            execute_code(
                r#"result = OpenIc.ext_index2identifier("n5yqx-uqaaa-aaaap-aatja-cai", 1);"#,
                r#"[]"#
            ),
            Ok(r#""ukbxy-zykor-uwiaa-aaaaa-dyae2-iaqca-aaaaa-q""#.into()) // cspell: disable-line
        );
        assert_eq!(
            execute_code(
                r#"result = OpenIc.ext_identifier2index("ukbxy-zykor-uwiaa-aaaaa-dyae2-iaqca-aaaaa-q");"#, // cspell: disable-line
                r#"[]"#
            ),
            Ok(r#"1"#.into())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            execute_code(
                r#"result = data.options.map((v) => ({ option: `${v} ICP`, value: `${v}` }));"#,
                r#"[["data","{\"options\":[0.1,0.2,0.5,1]}"]]"#
            ),
            Ok(r#"[{"option":"0.1 ICP","value":"0.1"},{"option":"0.2 ICP","value":"0.2"},{"option":"0.5 ICP","value":"0.5"},{"option":"1 ICP","value":"1"}]"#.into())
        );

        assert_eq!(
            execute_code(
                r#"result = data.options.map((v) => ({ option: `${v} ICP`, value: `${v}` }));"#,
                r#"[["data","{\"options\":[0.1,0.2,0.5,1]}"], ["data2",""]]"#
            ),
            Ok(r#"[{"option":"0.1 ICP","value":"0.1"},{"option":"0.2 ICP","value":"0.2"},{"option":"0.5 ICP","value":"0.5"},{"option":"1 ICP","value":"1"}]"#.into())
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            execute_validate_code(
                r#"result = data.length === 3 ? '' : 'wrong length';"#,
                r#"{"text":"xxx"}"#
            ),
            Ok(r#""""#.into())
        );
        assert_eq!(
            execute_validate_code(
                r#"result = data.length === 3 ? '' : 'wrong length';"#,
                r#"{"text":"xxxx"}"#
            ),
            Ok(r#""wrong length""#.into())
        );
        assert!(execute_validate_code(r#"let a : number = 1;"#, r#"{"text":"xxxx"}"#)
            .is_err_and(|e| matches!(e, ExecuteCodeError::ExecuteError(_))),);
    }

    #[test]
    fn test4() {
        assert_eq!(
            execute_code(
                r#"const fi = (n) => {
                    if (n === 1 || n === 2) return 1;
                    return fi(n - 1) + fi(n - 2);
                }
                result = fi(31);"#,
                "[]"
            ),
            Ok(r#"1346269"#.into())
        );
    }
}
