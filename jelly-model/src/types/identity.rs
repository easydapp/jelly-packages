use std::{borrow::Cow, marker::PhantomData};

use ic_stable_structures::{storable::Bound, Storable};

use crate::types::ContentHash;

/// string identity
pub struct StringIdentity<T> {
    inner: String,
    _tag: PhantomData<T>,
}

// Basic method

impl<T> Clone for StringIdentity<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _tag: PhantomData,
        }
    }
}

impl<T> Eq for StringIdentity<T> {}

impl<T> PartialEq for StringIdentity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Ord for StringIdentity<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> PartialOrd for StringIdentity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::hash::Hash for StringIdentity<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

// Not all string can be converted to ID
// impl<T> From<String> for StringIdentity<T> {
//     fn from(inner: String) -> Self {
//         StringIdentity {
//             inner,
//             _tag: PhantomData,
//         }
//     }
// }

impl<T> StringIdentity<T> {
    #[allow(unused)]
    pub(crate) fn from(inner: String) -> Self {
        StringIdentity {
            inner,
            _tag: PhantomData,
        }
    }
}

impl<T> From<StringIdentity<T>> for String {
    fn from(value: StringIdentity<T>) -> Self {
        value.inner
    }
}

impl<T> AsRef<String> for StringIdentity<T> {
    fn as_ref(&self) -> &String {
        &self.inner
    }
}

impl<T> serde::Serialize for StringIdentity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for StringIdentity<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: String::deserialize(deserializer)?,
            _tag: PhantomData::<T>,
        })
    }
}

// No need to save
// impl<T> Storable for StringIdentity<T> {
//     fn to_bytes(&self) -> Cow<[u8]> {
//         self.inner.to_bytes()
//     }

//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Self::from(String::from_bytes(bytes))
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

/// hash identity
pub struct HashIdentity<T> {
    inner: ContentHash,
    _tag: PhantomData<T>,
}

impl<T> Clone for HashIdentity<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _tag: PhantomData,
        }
    }
}

// Basic method

impl<T> Eq for HashIdentity<T> {}

impl<T> PartialEq for HashIdentity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Ord for HashIdentity<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> PartialOrd for HashIdentity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::hash::Hash for HashIdentity<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> From<ContentHash> for HashIdentity<T> {
    fn from(inner: ContentHash) -> Self {
        HashIdentity {
            inner,
            _tag: PhantomData,
        }
    }
}

impl<T> From<HashIdentity<T>> for ContentHash {
    fn from(value: HashIdentity<T>) -> Self {
        value.inner
    }
}

impl<T> AsRef<ContentHash> for HashIdentity<T> {
    fn as_ref(&self) -> &ContentHash {
        &self.inner
    }
}

impl<T> serde::Serialize for HashIdentity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for HashIdentity<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: ContentHash::deserialize(deserializer)?,
            _tag: PhantomData::<T>,
        })
    }
}

impl<T> Storable for HashIdentity<T> {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.inner.to_bytes()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ContentHash::from_bytes(bytes).into()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: true,
    };
}

/// id identity
pub struct U64Identity<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

// Basic method

impl<T> Clone for U64Identity<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _tag: PhantomData,
        }
    }
}

impl<T> Eq for U64Identity<T> {}

impl<T> PartialEq for U64Identity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Ord for U64Identity<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> PartialOrd for U64Identity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::hash::Hash for U64Identity<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> From<u64> for U64Identity<T> {
    fn from(inner: u64) -> Self {
        U64Identity {
            inner,
            _tag: PhantomData,
        }
    }
}

impl<T> From<U64Identity<T>> for u64 {
    fn from(value: U64Identity<T>) -> Self {
        value.inner
    }
}

impl<T> AsRef<u64> for U64Identity<T> {
    fn as_ref(&self) -> &u64 {
        &self.inner
    }
}

impl<T> serde::Serialize for U64Identity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for U64Identity<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: u64::deserialize(deserializer)?,
            _tag: PhantomData::<T>,
        })
    }
}

impl<T> Storable for U64Identity<T> {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.inner.to_bytes()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        u64::from_bytes(bytes).into()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
}
