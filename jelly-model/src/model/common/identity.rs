/// Component id
#[derive(Debug, Clone, Copy)]
pub struct ComponentId(u32);

impl Eq for ComponentId {}

impl PartialEq for ComponentId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for ComponentId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ComponentId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for ComponentId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl serde::Serialize for ComponentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ComponentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(u32::deserialize(deserializer)?))
    }
}

impl From<u32> for ComponentId {
    fn from(inner: u32) -> Self {
        Self(inner)
    }
}

impl ComponentId {
    /// Whether it is 0
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
