use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkError, LinkType};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text;
}

/// transfer
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EvmActionTransfer {
    /// If the target address is reference, it must be the text type // It must be in line with a string in 0xaaa format
    transfer_to: InputValue,

    pay_value: InputValue, // Whether to send tokens wei unit // text type

    #[serde(skip_serializing_if = "Option::is_none")]
    gas_price: Option<InputValue>, // gas price // text type

    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<InputValue>, // Whether to specify nonce // integer type
}

impl EvmActionTransfer {
    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check(&self, endpoints: &AllEndpoints<'_>, output: &LinkType, from: ComponentId) -> Result<Self, LinkError> {
        // 1. check transfer_to
        let transfer_to = self.transfer_to.check_transfer_to(endpoints, from)?;

        // 2. check pay_value
        let pay_value = self.pay_value.check_pay_value(endpoints, from)?;

        // 3. check gas_price
        let mut gas_price = None;
        if let Some(gas_price_ref) = &self.gas_price {
            gas_price = Some(gas_price_ref.check_gas_price(endpoints, from)?);
        }

        // 4. check nonce
        let mut nonce = None;
        if let Some(nonce_ref) = &self.nonce {
            nonce = Some(nonce_ref.check_nonce(endpoints, from)?);
        }

        // 10. check output
        if *output != *OUTPUT_LINK_TYPE {
            return Err(LinkError::InvalidCallEvmActionOutput(
                (from, "output must be text for transaction".into()).into(),
            ));
        }

        Ok(Self {
            transfer_to,
            pay_value,
            gas_price,
            nonce,
        })
    }
}
