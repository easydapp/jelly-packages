use crate::{
    model::{
        check::{check, find_all_anchors, find_origin_codes},
        common::{
            call_trigger::{CallTriggerLoading, ComponentCallTrigger},
            code::{CodeContent, OriginCodeContent},
            error::LinkError,
            lets::Endpoint,
            refer::{InputValue, ReferValue},
            types::LinkType,
            values::LinkValue,
        },
        components::{
            call::{
                http::{CallHttpMetadata, HttpMethod, ParsedWay},
                CallMetadata, ComponentCall,
            },
            code::{CodeMetadata, ComponentCode},
            condition::{
                bool::ConditionBoolCompare, text::ConditionTextCompare, ComponentCondition, Condition, ConditionItem,
                ConditionMatches, ConditionMetadata,
            },
            constant::{ComponentConst, ConstMetadata},
            form::ComponentForm,
            param::{ComponentParam, ParamMetadata},
            view::{text::ViewTextMetadata, ComponentView, ViewMetadata},
            LinkComponent,
        },
        types::check::{CheckFunction, CheckedCombined},
    },
    store::code::item::CodeItem,
};

struct MockCallFunction;

impl CheckFunction for MockCallFunction {
    fn canister_id(&self) -> Result<&str, String> {
        Ok("aaaaa-aa")
    }

    fn fetch_code(
        &self,
        _code_anchor: &crate::store::code::anchor::CodeDataAnchor,
    ) -> Result<&crate::store::code::CodeData, String> {
        unreachable!()
    }

    fn fetch_api(
        &self,
        _api_anchor: &crate::store::api::anchor::ApiDataAnchor,
    ) -> Result<&crate::store::api::ApiData, String> {
        unreachable!()
    }

    fn fetch_combined(
        &self,
        _combined_anchor: &crate::store::combined::anchor::CombinedAnchor,
    ) -> Result<&crate::store::combined::Combined, String> {
        unreachable!()
    }

    fn fetch_origin_api<'a, 'b: 'a>(&'a self, _key: &'b str) -> Result<&'a str, String> {
        unreachable!()
    }

    fn compile_code(&self, _item: &crate::store::code::item::CodeItem) -> Result<&str, String> {
        Ok("test")
    }
}

#[test]
fn test() {
    let fetch = MockCallFunction;
    let components = vec![
        LinkComponent::Param(ComponentParam {
            id: 1.into(),
            metadata: ParamMetadata {
                name: "name".into(),
                default: Some("Bob".into()),
            },
        }),
        LinkComponent::Const(ComponentConst {
            id: 2.into(),
            metadata: ConstMetadata {
                value: LinkValue::Text("Anubis".into()),
            },
            output: LinkType::Text,
        }),
        LinkComponent::Code(ComponentCode {
            id: 3.into(),
            inlets: Some(vec![Endpoint {
                id: 1.into(),
                index: None,
            }]),
            metadata: CodeMetadata {
                data: None,
                code: CodeContent::Code(OriginCodeContent {
                    code: CodeItem {
                        code: "x".into(),
                        args: None,
                        ret: None,
                    },
                    js: "".into(),
                }),
            },
            output: LinkType::Text,
        }),
    ];

    let checked = check(&components, &fetch);
    println!("{:#?}", checked);
    assert!(matches!(checked, Ok(CheckedCombined { .. })));
}

#[test]
fn test2() {
    let fetch = MockCallFunction;
    let components = vec![LinkComponent::Call(ComponentCall {
        id: 1.into(),
        inlets: Some(vec![Endpoint {
            id: 1.into(),
            index: None,
        }]),
        metadata: CallMetadata::Http(CallHttpMetadata {
            trigger: ComponentCallTrigger::Loading(CallTriggerLoading { alive: None }),
            identity: Some(1.into()),
            url: InputValue::Const(LinkValue::Text("123".into())),
            method: HttpMethod::Get,
            headers: None,
            body: None,
            parsed: ParsedWay::Text,
            post: None,
        }),
        output: LinkType::Text,
    })];

    let checked = check(&components, &fetch);

    println!("{:#?}", checked);
    assert!(matches!(checked, Err(LinkError::CircularReference { .. })));
}

#[test]
fn test3() {
    let fetch = MockCallFunction;
    let components = vec![LinkComponent::Call(ComponentCall {
        id: 1.into(),
        inlets: None,
        metadata: CallMetadata::Http(CallHttpMetadata {
            trigger: ComponentCallTrigger::Loading(CallTriggerLoading { alive: None }),
            identity: None,
            url: InputValue::Const(LinkValue::Text("https://123".into())),
            method: HttpMethod::Get,
            headers: None,
            body: None,
            parsed: ParsedWay::Text,
            post: None,
        }),
        output: LinkType::Text,
    })];

    let checked = check(&components, &fetch);
    println!("{:#?}", checked);
    assert!(matches!(checked, Ok(CheckedCombined { .. })));
    let checked = find_origin_codes(&components, &fetch);
    println!("{:#?}", checked);
    assert!(matches!(checked, Ok(_)));
    let checked = find_all_anchors(&components);
    println!("{:#?}", checked);
    assert!(matches!(checked, Ok(_)));
}

#[test]
fn test4() {
    let fetch = MockCallFunction;
    let components = vec![
        LinkComponent::Form(ComponentForm {
            id: 1.into(),
            inlets: None,
            metadata: None,
            output: LinkType::Text,
        }),
        LinkComponent::Const(ComponentConst {
            id: 2.into(),
            metadata: ConstMetadata {
                value: LinkValue::Bool(true),
            },
            output: LinkType::Bool,
        }),
        LinkComponent::Condition(ComponentCondition {
            id: 3.into(),
            inlets: Some(vec![
                Endpoint {
                    id: 2.into(),
                    index: None,
                },
                Endpoint {
                    id: 1.into(),
                    index: None,
                },
            ]),
            metadata: ConditionMetadata {
                conditions: vec![Condition::Required(ConditionItem {
                    value: ReferValue {
                        endpoint: Endpoint {
                            id: 2.into(),
                            index: None,
                        },
                        refer: None,
                    },
                    matches: ConditionMatches::Bool(ConditionBoolCompare::IsTrue),
                })],
            },
        }),
        LinkComponent::Code(ComponentCode {
            id: 10.into(),
            inlets: Some(vec![Endpoint {
                id: 3.into(),
                index: None,
            }]),
            metadata: CodeMetadata {
                data: None,
                code: CodeContent::Code(OriginCodeContent {
                    code: CodeItem {
                        code: "x".into(),
                        args: None,
                        ret: None,
                    },
                    js: "".into(),
                }),
            },
            output: LinkType::Text,
        }),
        LinkComponent::Code(ComponentCode {
            id: 20.into(),
            inlets: Some(vec![Endpoint {
                id: 3.into(),
                index: Some(1),
            }]),
            metadata: CodeMetadata {
                data: None,
                code: CodeContent::Code(OriginCodeContent {
                    code: CodeItem {
                        code: "x".into(),
                        args: None,
                        ret: None,
                    },
                    js: "".into(),
                }),
            },
            output: LinkType::Text,
        }),
        LinkComponent::Condition(ComponentCondition {
            id: 100.into(),
            inlets: Some(vec![
                Endpoint {
                    id: 10.into(),
                    index: None,
                },
                Endpoint {
                    id: 20.into(),
                    index: None,
                },
                Endpoint {
                    id: 1.into(),
                    index: None,
                },
            ]),
            metadata: ConditionMetadata {
                conditions: vec![Condition::Or(vec![
                    Condition::Required(ConditionItem {
                        value: ReferValue {
                            endpoint: Endpoint {
                                id: 10.into(),
                                index: None,
                            },
                            refer: None,
                        },
                        matches: ConditionMatches::Text(ConditionTextCompare::NotNull),
                    }),
                    Condition::Required(ConditionItem {
                        value: ReferValue {
                            endpoint: Endpoint {
                                id: 20.into(),
                                index: None,
                            },
                            refer: None,
                        },
                        matches: ConditionMatches::Text(ConditionTextCompare::NotNull),
                    }),
                ])],
            },
        }),
        LinkComponent::View(ComponentView {
            id: 200.into(),
            inlets: Some(vec![Endpoint {
                id: 100.into(),
                index: None,
            }]),
            metadata: ViewMetadata::Text(ViewTextMetadata {
                value: InputValue::Refer(ReferValue {
                    endpoint: Endpoint {
                        id: 1.into(),
                        index: None,
                    },
                    refer: None,
                }),
                href: None,
                style: None,
            }),
        }),
    ];

    let checked = check(&components, &fetch);
    println!("{:#?}", checked);
    assert!(matches!(checked, Ok(CheckedCombined { .. })));
}
