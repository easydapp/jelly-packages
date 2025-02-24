/// Timestamp millisecond
#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct TimestampMills(pub i64);

impl std::ops::Add<i64> for TimestampMills {
    type Output = i64;

    fn add(self, rhs: i64) -> Self::Output {
        self.0 + rhs
    }
}

impl PartialOrd for TimestampMills {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<i64> for TimestampMills {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<TimestampMills> for i64 {
    fn from(value: TimestampMills) -> Self {
        value.0
    }
}

impl AsRef<i64> for TimestampMills {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl core::fmt::Display for TimestampMills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl serde::Serialize for TimestampMills {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for TimestampMills {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(i64::deserialize(deserializer)?))
    }
}

#[cfg(feature = "wasm_bindgen")]
mod js_value {
    use super::TimestampMills;

    impl From<TimestampMills> for wasm_bindgen::JsValue {
        fn from(value: TimestampMills) -> Self {
            wasm_bindgen::JsValue::from_f64(value.0 as f64) // ! JS's maximum support is 2^53-1
        }
    }
}
