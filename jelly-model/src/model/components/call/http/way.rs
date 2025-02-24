/// Analysis of the request result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedWay {
    /// blob()
    Blob,
    /// json()
    Json,
    /// text()
    Text,
}

impl serde::Serialize for ParsedWay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ParsedWay::Blob => "blob",
            ParsedWay::Json => "json",
            ParsedWay::Text => "text",
        }
        .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ParsedWay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = match s.as_str() {
            "blob" => ParsedWay::Blob,
            "json" => ParsedWay::Json,
            "text" => ParsedWay::Text,
            _ => return Err(serde::de::Error::custom("invalid http parsed way")),
        };
        Ok(value)
    }
}
