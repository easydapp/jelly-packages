use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::types::CallChain;

use super::{AllEndpoints, ComponentId, ComponentTriggered, EvmChain, EvmWallet, InputValue, LinkError, LinkType};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::object_builder()
        .push("chain", LinkType::Text) // chain
        .push("chain_id", LinkType::Integer) // chain id
        .push("wallet", LinkType::Text) // wallet
        .push("account", LinkType::Text) // account 0xaaa...
        .build();
}

/// evm metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IdentityEvmMetadata {
    /// chain
    pub chain: EvmChain,

    /// Supported wallet // ! Wallets that do not specify any support indicate anonymous calls
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    includes: Option<Vec<EvmWallet>>,

    /// Exclusive wallet
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    excludes: Option<Vec<EvmWallet>>,

    /// Whether to display the button
    #[serde(skip_serializing_if = "Option::is_none")]
    connect: Option<InputValue>,
}

impl IdentityEvmMetadata {
    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        Cow::Borrowed(&OUTPUT_LINK_TYPE)
    }

    /// Whether to be anonymous
    pub fn is_anonymous(&self) -> bool {
        crate::is_empty_option_vec(&self.includes)
    }

    /// check
    pub fn check(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        from: ComponentId,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
    ) -> Result<Self, LinkError> {
        // * 0. Determine whether the wallet supports the current network
        if let Some(includes) = self.includes.as_ref() {
            for wallet in includes {
                if !EvmWallet::get_supported_chain(wallet).contains(&self.chain) {
                    return Err(LinkError::InvalidIdentity(
                        (from, format!("evm identity includes not support chain: {:?}", wallet)).into(),
                    ));
                }
            }
        }

        let includes = self
            .includes
            .as_ref()
            .map(|includes| includes.iter().map(EvmWallet::text).collect::<Vec<_>>());
        let excludes = self
            .excludes
            .as_ref()
            .map(|excludes| excludes.iter().map(EvmWallet::text).collect::<Vec<_>>());

        // 1. Check the allowable wallet
        if let Some(includes) = includes.as_ref() {
            // There must be a allowable wallet
            if includes.is_empty() {
                return Err(LinkError::InvalidIdentity(
                    (from, "evm identity includes can not be empty".into()).into(),
                ));
            }
            // Wallets are not allowed to repeat
            if includes.iter().collect::<HashSet<_>>().len() != includes.len() {
                return Err(LinkError::InvalidIdentity(
                    (from, "evm identity includes can not be repeated".into()).into(),
                ));
            }
        }

        // 2. Check the exclusive wallet
        if let Some(excludes) = excludes.as_ref() {
            // There must be a wallet that must be excluded, otherwise it should be NONE
            if excludes.is_empty() {
                return Err(LinkError::InvalidIdentity(
                    (from, "evm identity excludes can not be empty".into()).into(),
                ));
            }
            // Wallets are not allowed to repeat
            if excludes.iter().collect::<HashSet<_>>().len() != excludes.len() {
                return Err(LinkError::InvalidIdentity(
                    (from, "evm identity excludes can not be repeated".into()).into(),
                ));
            }
            // The excreted wallet cannot be repeated with the allowable wallet
            if let Some(includes) = includes.as_ref() {
                let includes = includes.iter().collect::<HashSet<_>>();
                if !includes.is_disjoint(&excludes.iter().collect::<HashSet<_>>()) {
                    return Err(LinkError::InvalidIdentity(
                        (from, "evm identity includes and excludes can not be intersect".into()).into(),
                    ));
                }
            }
        }

        // 3. Check the link button
        if let Some(connect) = self.connect.as_ref() {
            let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
            connect.check_text_input_value(
                &endpoints,
                |text| !text.trim().is_empty(),
                "connect identity button text can not be empty",
                "connect identity button text can not be text",
                "connect identity button text can not be text type",
                LinkError::InvalidIdentity,
                from,
            )?;
        }

        // 4. Anonymous wallet is not allowed to be triggered
        if self.is_anonymous() && self.connect.is_some() {
            return Err(LinkError::InvalidIdentity(
                (from, "connect identity button can not be set when anonymous".into()).into(),
            ));
        }

        // 5. Recording trigger
        triggers.insert(from, ComponentTriggered::from_identity(from, self.connect.is_some()));

        Ok(self.clone())
    }

    /// get call chain
    pub fn get_call_chain(&self) -> CallChain {
        self.chain.get_call_chain()
    }
}
