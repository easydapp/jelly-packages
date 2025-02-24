use crate::types::ContentHash;

/// sha256
pub fn hash_sha256(value: &str) -> ContentHash {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(value);
    let result = hasher.finalize();
    result.into()
    // format!("{:x}", result)
}
