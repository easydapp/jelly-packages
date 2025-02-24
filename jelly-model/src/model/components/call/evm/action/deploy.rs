use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeType, CodeValue, ComponentId, InputValue, LinkError, LinkType,
};

/// evm action deploy initial
pub mod initial;

use initial::EvmDeployInitial;

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::object_builder()
        .push("tx", LinkType::Text) // tx
        .push("address", LinkType::Text) // address
        .build();
}

/// Deployment contract
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EvmActionDeploy {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pay_value: Option<InputValue>, // Whether to send tokens wei unit // text type // ! Deploying contracts cannot be accepted for transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    gas_limit: Option<InputValue>, // gas limit // integer type

    #[serde(skip_serializing_if = "Option::is_none")]
    gas_price: Option<InputValue>, // gas price // text type

    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<InputValue>, // Whether to specify nonce // integer type

    abi: InputValue, // Deploy contract code // text type json

    bytecode: InputValue, // Deploy contract code // text type hex

    #[serde(skip_serializing_if = "Option::is_none")]
    initial: Option<EvmDeployInitial>, // Construction parameters of contract code
}

impl EvmActionDeploy {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        // initial
        if let Some(initial) = &self.initial {
            anchors.extend(initial.get_code_anchors());
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        // initial
        if let Some(initial) = &self.initial {
            codes.extend(initial.get_origin_codes(endpoints, from)?);
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // 1. check gas_limit
        let mut gas_limit = None;
        if let Some(gas_limit_ref) = &self.gas_limit {
            gas_limit = Some(gas_limit_ref.check_gas_limit(endpoints, from)?);
        }

        // 2. check gas_price
        let mut gas_price = None;
        if let Some(gas_price_ref) = &self.gas_price {
            gas_price = Some(gas_price_ref.check_gas_price(endpoints, from)?);
        }

        // 3. check nonce
        let mut nonce = None;
        if let Some(nonce_ref) = &self.nonce {
            nonce = Some(nonce_ref.check_nonce(endpoints, from)?);
        }

        // 4. check abi
        let abi = self.abi.check_abi(endpoints, from)?;

        // 5. check bytecode
        let bytecode = self.bytecode.check_bytecode(endpoints, from)?;

        // 6. check initial
        let mut initial = None;
        if let Some(initial_ref) = &self.initial {
            initial = Some(initial_ref.check(endpoints, from, fetch, codes)?);
        }

        // 7. check output
        if *output != *OUTPUT_LINK_TYPE {
            return Err(LinkError::InvalidCallEvmActionOutput(
                (
                    from,
                    "output must be { tx: string, address: string } for deploying".into(),
                )
                    .into(),
            ));
        }

        Ok(Self {
            gas_limit,
            gas_price,
            nonce,
            abi,
            bytecode,
            initial,
        })
    }
}
