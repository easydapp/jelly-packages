use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeType, CodeValue, ComponentCallTrigger, ComponentId, ComponentTriggered, IdentityInnerMetadata, InputValue,
    LinkError, LinkType, NamedValue,
};

mod method;

mod body;

mod way;

pub use method::HttpMethod;

pub use body::{HttpBody, HttpBodyCode, HttpBodyPlain};

pub use way::ParsedWay;

/// http metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallHttpMetadata {
    /// Trigger condition
    pub trigger: ComponentCallTrigger,

    /// The required identity empty indicates the use of anonymous identity
    /// It must be IdentityHttpMetadata type node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ComponentId>,

    /// If the target address is reference, it must be the text type
    pub url: InputValue,

    /// Method of requesting
    pub method: HttpMethod,

    /// Request
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub headers: Option<Vec<NamedValue>>,

    /// Request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<HttpBody>,

    /// Analytical method
    pub parsed: ParsedWay,

    /// Don’t know what to return after the processing, so I can only use code processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<CodeContent>,
}

impl CallHttpMetadata {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        // body
        if let Some(body) = &self.body {
            anchors.extend(body.get_code_anchors());
        }

        // post
        if let Some(code) = &self.post {
            anchors.extend(code.get_code_anchors());
        }

        anchors
    }

    /// Calculate the parameters of the code after calculating
    #[inline]
    fn get_post_args(
        endpoints: &AllEndpoints<'_>,
        data: CodeType,
        headers: &Option<Vec<NamedValue>>,
        body: &Option<HttpBody>,
        data_of_request_body: CodeType,
        from: ComponentId,
    ) -> Result<Option<Vec<ArgCodeType>>, LinkError> {
        let request_body = if let Some(body) = body {
            match body {
                HttpBody::Plain(HttpBodyPlain { data }) => {
                    // Check the introduction variable
                    let data = endpoints.check_named_values(
                        data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(),
                        from,
                        None,
                    )?;
                    CodeType::from_ty(data.typescript())
                }
                HttpBody::Code(_) => CodeType::any(),
            }
        } else {
            CodeType::undefined()
        };
        Ok(Some(vec![
            ArgCodeType::from("data", data),
            ArgCodeType::from("response_headers", CodeType::from_ty("[string, string][]")),
            ArgCodeType::from("request_url", CodeType::from_ty("string")),
            ArgCodeType::from(
                "request_method",
                CodeType::from_ty("('GET' | 'POST' | 'PUT' | 'DELETE')"),
            ),
            ArgCodeType::from(
                "request_headers",
                headers
                    .as_ref()
                    .map(|_| CodeType::from_ty("[string, string][]"))
                    .unwrap_or(CodeType::undefined()),
            ),
            ArgCodeType::from("request_body", request_body),
            ArgCodeType::from("data_of_request_body", data_of_request_body),
        ]))
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        output: &LinkType,
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        // 0 Check whether the reference is matched
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        let mut data_of_request_body: CodeType = CodeType::undefined();

        // body
        if let Some(body) = &self.body {
            codes.extend(body.get_origin_codes(&endpoints, from, 0, "Http -> body", |arg| data_of_request_body = arg)?);
        }

        if let Some(post) = &self.post {
            if let Some(code) = post.get_origin_code() {
                // Calculate parameter type and output type
                let data = match self.parsed {
                    ParsedWay::Blob => CodeType::from_ty("number[]"),
                    ParsedWay::Json => CodeType::any(), // Too complicated
                    ParsedWay::Text => CodeType::from_ty("string"),
                };
                let output = CodeType::from_ty(output.typescript()); // HTTP interface request data required, no need to check

                codes.push(CheckedCodeItem::new(
                    from,
                    1,
                    "Http -> post".into(),
                    CodeItem {
                        code,
                        args: Self::get_post_args(
                            &endpoints,
                            data,
                            &self.headers,
                            &self.body,
                            data_of_request_body,
                            from,
                        )?,
                        ret: Some(output),
                    },
                ));
            }
        }

        Ok(codes)
    }

    /// Check the output type
    #[inline]
    fn check_output_by_post(&self, output: &LinkType, from: ComponentId) -> Result<(), LinkError> {
        if self.post.is_none() {
            match &self.parsed {
                ParsedWay::Blob => {
                    if !output.is_blob() {
                        return Err(LinkError::InvalidCallOutputType(
                            (from, "output type must be blob".into()).into(),
                        ));
                    }
                }
                ParsedWay::Json => {} // There are too many types of support, do not check
                ParsedWay::Text => {
                    if !output.is_text() {
                        return Err(LinkError::InvalidCallOutputType(
                            (from, "output type must be text".into()).into(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Check whether the component is effective
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<CallHttpMetadata, LinkError> {
        // 1 check trigger
        let trigger = self.trigger.check(endpoints, from)?;

        // 2. Check identity
        let mut identity = None;
        if let Some(identity_ref) = self.identity {
            identity = Some(identity_ref.check_identity(
                endpoints,
                |metadata| matches!(metadata, IdentityInnerMetadata::Http(_)),
                "identity component is not http",
                |_, _| Ok(()),
                from,
            )?);
        }

        // 3. check url
        let url = self.url.check_text_input_value(
            endpoints,
            |url| url.starts_with("https://"),
            "wrong constant url",
            "wrong constant url value",
            "wrong url type",
            LinkError::InvalidCallHttpUrl,
            from,
        )?;

        // 4. check method
        let method = self.method.clone();

        // 5. Recording trigger
        triggers.insert(
            from,
            ComponentTriggered::from_call(
                from,
                identity,
                matches!(trigger, ComponentCallTrigger::Click { .. }),
                matches!(method, HttpMethod::Post | HttpMethod::Put | HttpMethod::Delete),
            ),
        );

        // 6. check headers
        if let Some(headers) = &self.headers {
            endpoints.check_named_values(headers.iter(), from, Some(LinkType::Text))?;
        }
        let headers = self.headers.clone();

        // 7. check body
        let mut data_of_request_body: CodeType = CodeType::undefined();
        let mut body = None;
        if let Some(http_body) = &self.body {
            body = Some(http_body.check(endpoints, from, fetch, codes, |arg| data_of_request_body = arg)?);
        }

        // 8. check parsed way
        let parsed = self.parsed.clone();

        // 9. check post
        let mut post = None;
        if let Some(code) = &self.post {
            post = Some(code.check_post(
                endpoints,
                &headers,
                &body,
                data_of_request_body,
                &parsed,
                output,
                from,
                fetch,
                codes,
            )?);
        }

        // 10. Check the type returned
        self.check_output_by_post(output, from)?;

        Ok(Self {
            trigger,
            identity,
            url,
            method,
            headers,
            body,
            parsed,
            post,
        })
    }
}

impl CodeContent {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn check_post<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        headers: &Option<Vec<NamedValue>>,
        body: &Option<HttpBody>,
        data_of_request_body: CodeType,
        parsed: &ParsedWay,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // Calculate parameter type and output type
        let data = match parsed {
            ParsedWay::Blob => CodeType::from_ty("number[]"),
            ParsedWay::Json => CodeType::any(), // Too complicated
            ParsedWay::Text => CodeType::from_ty("string"),
        };
        let data = CallHttpMetadata::get_post_args(endpoints, data, headers, body, data_of_request_body, from)?;
        let output = CodeType::from_ty(output.typescript()); // HTTP interface request data required, no need to check

        let code = self.clone().try_into_anchor(data, Some(output), from, fetch, codes)?;

        Ok(code)
    }
}
