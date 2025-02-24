use serde::{Deserialize, Serialize};

use super::{
    error::{CommonLinkError, LinkError},
    identity::ComponentId,
    lets::{AllEndpoints, Endpoint},
    types::{LinkType, ObjectSubitem},
    values::LinkValue,
};

/// Any variable name variable introduction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NamedValue {
    /// name
    pub name: String,
    /// value
    pub value: InputValue,
}

/// Need to add calculated variables to introduce
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CodeValue {
    /// key //! Must be in line with variable naming rules
    pub key: String,
    /// value
    pub value: InputValue,
}

/// The value required for each position of each component may be constant, or it may be the value introduced through Inlets
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum InputValue {
    /// Constant value
    #[serde(rename = "const")]
    Const(LinkValue),

    /// Component dependency acquisition value
    #[serde(rename = "refer")]
    Refer(ReferValue),
}

impl InputValue {
    /// Determine whether it is quoted
    pub fn is_refer(&self) -> bool {
        matches!(self, Self::Refer(_))
    }

    /// Uniform inspection reference
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check_text_input_value<F, E>(
        &self,
        endpoints: &AllEndpoints<'_>,
        check: F,             // Check the function
        check_err: &str,      // Check the error message
        const_type_err: &str, // The constant type error must be Text
        refer_type_err: &str, // Quote Type errors must be Text
        err: E,               // Constructive object
        from: ComponentId,
    ) -> Result<Self, LinkError>
    where
        F: Fn(&str) -> bool,                 // Check the function
        E: Fn(CommonLinkError) -> LinkError, // Constructive object
    {
        match self {
            InputValue::Const(constant) => {
                if let LinkValue::Text(constant) = constant {
                    let text = constant.trim();
                    if !check(text) {
                        return Err(err((from, format!("{check_err}: {constant}")).into()));
                    }
                } else {
                    return Err(err((from, format!("{const_type_err}: {constant:?}")).into()));
                }
            }
            InputValue::Refer(refer) => {
                let output = endpoints.find_output_type(&refer.endpoint, &refer.refer, from)?;
                if !output.is_text() {
                    return Err(err((from, format!("{refer_type_err}: {:?}", output)).into()));
                }
            }
        }
        Ok(self.clone())
    }

    /// Uniform inspection reference
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check_integer_input_value<F, E>(
        &self,
        endpoints: &AllEndpoints<'_>,
        check: F,             // Check the function
        check_err: &str,      // Check the error message
        const_type_err: &str, // The constant type error must be Text
        refer_type_err: &str, // Quote Type errors must be Text
        err: E,               // Constructive object
        from: ComponentId,
    ) -> Result<Self, LinkError>
    where
        F: Fn(&i64) -> bool,                 // Check the function
        E: Fn(CommonLinkError) -> LinkError, // Constructive object
    {
        match self {
            InputValue::Const(constant) => {
                if let LinkValue::Integer(constant) = constant {
                    let integer = constant;
                    if !check(integer) {
                        return Err(err((from, format!("{check_err}: {constant}")).into()));
                    }
                } else {
                    return Err(err((from, format!("{const_type_err}: {constant:?}")).into()));
                }
            }
            InputValue::Refer(refer) => {
                let output = endpoints.find_output_type(&refer.endpoint, &refer.refer, from)?;
                if !output.is_integer() {
                    return Err(err((from, format!("{refer_type_err}: {:?}", output)).into()));
                }
            }
        }
        Ok(self.clone())
    }
}

/// Get value through component dependencies
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ReferValue {
    /// Which output of which component // ? A certain component may point multiple outputs to this component
    pub endpoint: Endpoint,
    /// Do you need recursively specifying key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer: Option<KeyRefer>,
}

/// Introduce the specified variable through key
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct KeyRefer {
    /// key
    key: String,
    /// Do you need recursively specifying key
    #[serde(skip_serializing_if = "Option::is_none")]
    refer: Option<Box<KeyRefer>>,
}

impl KeyRefer {
    /// Query introduction point, additional key needs to recursively traversing search
    pub fn get_output<'a>(
        &self,
        ty: &'a LinkType,
        from: &ComponentId,
        inlet: &Endpoint,
        tip: &KeyRefer, // The top Key, the error must be prompted
    ) -> Result<&'a LinkType, LinkError> {
        if let LinkType::Object(object) = ty {
            if let Some(ObjectSubitem { ty, .. }) = object.iter().find(|item| item.key == self.key) {
                if let Some(inner) = &self.refer {
                    return inner.get_output(ty, from, inlet, tip);
                } else {
                    return Ok(ty);
                }
            }
        }

        Err(LinkError::WrongLinkTypeForRefer {
            from: *from,
            inlet: *inlet,
            refer: tip.clone(),
        })
    }
}
