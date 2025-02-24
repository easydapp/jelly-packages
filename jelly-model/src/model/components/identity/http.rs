use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{ComponentId, ComponentTriggered, LinkError, LinkType};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::object_builder()
        .push("proxy", LinkType::Text) // Object, there is only one proxy inside
        .build();
}

/// http metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IdentityHttpMetadata {
    /// The proxy of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<String>,
}

impl IdentityHttpMetadata {
    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        Cow::Borrowed(&OUTPUT_LINK_TYPE)
    }

    /// check
    pub fn check(
        &self,
        from: ComponentId,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
    ) -> Result<Self, LinkError> {
        // 1. Check the proxy address
        if let Some(proxy) = self.proxy.as_ref() {
            match proxy.as_str() {
                "https://p.easydapp.ai" => {}
                _ => {
                    return Err(LinkError::InvalidIdentityHttpProxy {
                        from,
                        proxy: proxy.clone(),
                    })
                }
            }
        }

        // 2. Recording trigger
        triggers.insert(from, ComponentTriggered::from_identity(from, false));

        Ok(self.clone())
    }
}
