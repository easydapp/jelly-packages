use ic_canister_kit::{
    candid::parse_service_candid,
    types::{WrappedCandidType, WrappedCandidTypeFunction},
};

use crate::model::common::{error::LinkError, identity::ComponentId};

/// Analysis parameter type
pub(crate) fn parse_candid_for_data_and_output(
    candid: &str,
    method: Option<&str>,
    from: ComponentId,
) -> Result<WrappedCandidTypeFunction, LinkError> {
    fn error(from: ComponentId, candid: &str, message: String) -> LinkError {
        LinkError::CompileCallIcCandid {
            from,
            candid: candid.to_string(),
            message,
        }
    }

    let service = parse_service_candid(candid).map_err(|err| error(from, candid, format!("{err:?}")))?;
    let methods = service.methods;

    match method {
        Some(method) => {
            // designation method
            let found = methods.into_iter().find(|(m, _)| m == method);
            match found {
                Some((_method, func)) => Ok(func),
                None => Err(error(from, candid, format!("can not find method: {method}"))),
            }
        }
        None => {
            // Must be the only Method
            if 1 < methods.len() {
                return Err(error(from, candid, "service must has one method".into()));
            }
            let method = methods.into_iter().next();
            match method {
                Some((_method, func)) => Ok(func),
                None => Err(error(from, candid, "service is empty".into())),
            }
        }
    }
}

/// Interface parameter type
pub(crate) enum IcFunctionArgsType {
    /// No parameter
    None,
    /// Single parameter
    Single {
        /// Whether it can be omitted
        opt: bool,
    },
    /// Multiple and parameters
    Multi {
        /// Whether it can omit the parameters
        opt: bool,
    },
}

/// Whether the parameter can empty
pub(crate) fn types_args_type(items: &[WrappedCandidType]) -> IcFunctionArgsType {
    if items.is_empty() {
        return IcFunctionArgsType::None;
    }

    if items.len() == 1 {
        return IcFunctionArgsType::Single {
            opt: matches!(items[0], WrappedCandidType::Opt(_)),
        };
    }

    for item in items {
        if !matches!(item, WrappedCandidType::Opt(_)) {
            return IcFunctionArgsType::Multi { opt: false };
        }
    }

    IcFunctionArgsType::Multi { opt: true }
}
