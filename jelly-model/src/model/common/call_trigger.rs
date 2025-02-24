use serde::{Deserialize, Serialize};

use super::{error::LinkError, identity::ComponentId, lets::AllEndpoints, refer::InputValue};

/// call trigger loading
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallTriggerLoading {
    /// Cache time  default120000ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alive: Option<u32>,
}

impl CallTriggerLoading {
    /// Trigger inspection
    #[inline]
    fn check(&self) -> Result<Self, LinkError> {
        Ok(self.clone())
    }
}

/// call trigger clock
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallTriggerClock {
    /// Sleep time is also cache time
    pub sleep: u32,
    /// Whether to load the page If there is no default TRUE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading: Option<bool>,
}

impl CallTriggerClock {
    /// Trigger inspection
    #[inline]
    fn check(&self, from: ComponentId) -> Result<Self, LinkError> {
        if self.sleep < 10000 {
            return Err(LinkError::InvalidCallTrigger(
                (
                    from,
                    format!("sleep time must be greater than 10000ms, but got {}", self.sleep),
                )
                    .into(),
            ));
        }
        Ok(self.clone())
    }
}

/// call trigger click
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallTriggerClick {
    /// Click the display button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<InputValue>,
}

impl CallTriggerClick {
    /// Trigger inspection
    #[inline]
    fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        if let Some(text) = &self.text {
            text.check_text_input_value(
                endpoints,
                |text| !text.trim().is_empty(),
                "click text must not be empty",
                "click text must not be text value",
                "click text must not be text type",
                LinkError::InvalidCallTrigger,
                from,
            )?;
        }
        Ok(self.clone())
    }
}

/// Call the trigger condition
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ComponentCallTrigger {
    /// No need to trigger, automatic execution
    #[serde(rename = "loading")]
    Loading(CallTriggerLoading),

    /// Regularly execute
    #[serde(rename = "clock")]
    Clock(CallTriggerClock),

    /// Click to execute
    #[serde(rename = "click")]
    Click(CallTriggerClick),
}

impl ComponentCallTrigger {
    /// Trigger inspection
    #[inline]
    pub fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        let trigger = match self {
            Self::Loading(loading) => Self::Loading(loading.check()?),
            Self::Clock(clock) => Self::Clock(clock.check(from)?),
            Self::Click(click) => Self::Click(click.check(endpoints, from)?),
        };

        Ok(trigger)
    }
}
