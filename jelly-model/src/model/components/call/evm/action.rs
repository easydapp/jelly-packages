use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::check::{is_valid_evm_address, is_valid_hex_text};

use super::{
    AbiItem, AbiParam, AbiStateMutability, AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction,
    CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem, CodeType, CodeValue, ComponentId, EvmCallApi,
    InputValue, LinkError, LinkType,
};

/// emv action call
pub mod call;

/// emv action transaction
pub mod transaction;

/// emv action deploy
pub mod deploy;

/// emv action transfer
pub mod transfer;

use call::{EvmActionCall, EvmCallArg};

use transaction::EvmActionTransaction;

use deploy::EvmActionDeploy;

use transfer::EvmActionTransfer;

/// EVM call behavior
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmAction {
    /// Call contract
    #[serde(rename = "call")]
    Call(EvmActionCall),
    /// Use your identity to sign
    #[serde(rename = "sign")]
    Sign(InputValue),
    /// Transaction
    #[serde(rename = "transaction")]
    Transaction(EvmActionTransaction),
    /// Deployment contract
    #[serde(rename = "deploy")]
    Deploy(EvmActionDeploy),
    /// transfer
    #[serde(rename = "transfer")]
    Transfer(EvmActionTransfer),
}

impl EvmAction {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            EvmAction::Call(call) => anchors.extend(call.get_code_anchors()),
            EvmAction::Sign(_) => {} // Signature no code
            EvmAction::Transaction(transaction) => anchors.extend(transaction.get_code_anchors()),
            EvmAction::Deploy(deploy) => anchors.extend(deploy.get_code_anchors()),
            EvmAction::Transfer(_) => {} // There is no code for transfer
        }

        anchors
    }

    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            EvmAction::Call(call) => anchors.extend(call.get_apis_anchors()),
            EvmAction::Sign(_) => {} // Signature no API
            EvmAction::Transaction(transaction) => anchors.extend(transaction.get_apis_anchors()),
            EvmAction::Deploy(_) => {}   // Deploying without API
            EvmAction::Transfer(_) => {} // There is no API for transfer
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        match self {
            EvmAction::Call(call) => codes.extend(call.get_origin_codes(endpoints, output, from, fetch)?),
            EvmAction::Sign(_) => {} // Signature no code
            EvmAction::Transaction(transaction) => {
                codes.extend(transaction.get_origin_codes(endpoints, output, from, fetch)?)
            }
            EvmAction::Deploy(deploy) => codes.extend(deploy.get_origin_codes(endpoints, from)?),
            EvmAction::Transfer(_) => {} // There is no code for transfer
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
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<Self, LinkError> {
        let action = match self {
            Self::Call(call) => Self::Call(call.check(endpoints, output, from, fetch, codes, apis)?),
            Self::Sign(sign) => Self::Sign(sign.check_sign(endpoints, output, from)?),
            Self::Transaction(transaction) => {
                Self::Transaction(transaction.check(endpoints, output, from, fetch, codes, apis)?)
            }
            Self::Deploy(deploy) => Self::Deploy(deploy.check(endpoints, output, from, fetch, codes)?),
            Self::Transfer(transfer) => Self::Transfer(transfer.check(endpoints, output, from)?),
        };

        Ok(action)
    }
}

impl InputValue {
    #[inline]
    fn check_contract(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            is_valid_evm_address,
            "wrong contract address",
            "wrong contract address value",
            "wrong contract address type",
            LinkError::InvalidCallEvmActionContract,
            from,
        )
    }

    /// Check whether the component is effective
    #[inline]
    fn check_sign(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
    ) -> Result<Self, LinkError> {
        // 1. check message
        let sign = self.check_text_input_value(
            endpoints,
            |message| !message.is_empty(),
            "wrong message",
            "wrong message value",
            "wrong message type",
            LinkError::InvalidCallEvmActionSign,
            from,
        )?;

        use lazy_static::lazy_static;
        lazy_static! {
            static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text;
        }

        // 2. Check the output type
        if *output != *OUTPUT_LINK_TYPE {
            return Err(LinkError::InvalidCallEvmActionSign(
                (from, "output must be text for sign action".into()).into(),
            ));
        }

        Ok(sign)
    }

    #[inline]
    fn check_pay_value(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            |pay_value| evm_value_exp(pay_value, 18).is_some(),
            "wrong pay value",
            "wrong pay value value",
            "wrong pay value type",
            LinkError::InvalidCallEvmActionPayValue,
            from,
        )
    }

    #[inline]
    fn check_gas_limit(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_integer_input_value(
            endpoints,
            |gas_limit| 0 < *gas_limit,
            "wrong gas limit",
            "wrong gas limit value",
            "wrong gas limit type",
            LinkError::InvalidCallEvmActionGasLimit,
            from,
        )
    }

    #[inline]
    fn check_gas_price(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            |gas_price| evm_value_exp(gas_price, 9).is_some(),
            "wrong gas price",
            "wrong gas price value",
            "wrong gas price type",
            LinkError::InvalidCallEvmActionGasPrice,
            from,
        )
    }

    #[inline]
    fn check_nonce(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_integer_input_value(
            endpoints,
            |nonce| 0 <= *nonce,
            "wrong nonce",
            "wrong nonce value",
            "wrong nonce type",
            LinkError::InvalidCallEvmActionNonce,
            from,
        )
    }

    #[inline]
    fn check_abi(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            |abi| serde_json::from_str::<Vec<AbiItem>>(abi).is_ok(),
            "wrong abi",
            "wrong abi value",
            "wrong abi type",
            LinkError::InvalidCallEvmActionAbi,
            from,
        )
    }

    #[inline]
    fn check_bytecode(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            is_valid_hex_text,
            "wrong bytecode",
            "wrong bytecode value",
            "wrong bytecode type",
            LinkError::InvalidCallEvmActionBytecode,
            from,
        )
    }

    #[inline]
    fn check_transfer_to(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<InputValue, LinkError> {
        self.check_text_input_value(
            endpoints,
            is_valid_evm_address,
            "wrong transfer to address",
            "wrong transfer to address value",
            "wrong transfer to address type",
            LinkError::InvalidCallEvmActionTransferTo,
            from,
        )
    }
}

// Determine the effectiveness of the string numbers
fn evm_value_exp(value: &str, exp: usize) -> Option<String> {
    let s = value.split('.').map(|v| v.to_string()).collect::<Vec<_>>();
    if 2 < s.len() {
        return None; // Multiple decimal points are not allowed
    }
    if 1 < s.len() && exp < s[1].len() {
        return None; // The number after decimal point exceeds the limit
    }
    let mut value = value.to_string();
    if !value.contains('.') {
        value += ".0"; // Add without a decimal point
    }
    loop {
        if let Some(v) = value.split('.').last() {
            if exp <= v.len() {
                break;
            }
        } else {
            return None;
        }
        value += "0"; // Required 0
    }
    let value = value
        .replace(".", "")
        .chars()
        .skip_while(|c| *c == '0') // Skip the front 0
        .collect::<String>(); // Remove the decimal point
    let v = value.parse::<u128>().ok()?;
    if format!("{v}") != value {
        return None; // Can you restore the original number
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", evm_value_exp("123", 18));
        println!("{:?}", evm_value_exp("123.0", 18));
        println!("{:?}", evm_value_exp("123.01", 18));
        println!("{:?}", evm_value_exp("123.0000000000000000000001", 18));
        println!("{:?}", evm_value_exp("0.00001", 18));
    }
}
