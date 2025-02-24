/// Method of requesting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    /// GET request
    Get,
    /// post request
    Post,
    /// PUT request
    Put,
    /// delete request
    Delete,
}

impl serde::Serialize for HttpMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
        .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for HttpMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = match s.as_str() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            _ => return Err(serde::de::Error::custom("invalid http method")),
        };
        Ok(value)
    }
}
