use candid::Principal;

use crate::{
    common::hash::hash_sha256,
    types::{StringIdentity, U64Identity},
};

use super::Dapp;

/// Prefix
const PREFIX: &str = "in";

/// share id
pub type DappAnchor = StringIdentity<Dapp>;

/// hash id
pub type DappId = U64Identity<Dapp>;

/// parsed id
pub struct DappParsedId {
    /// canister
    pub canister_id: Principal,
    /// id
    pub id: DappId,
    /// The number of releases starts from 1, no or 0 means the latest version
    pub nonce: Option<u32>,
}

impl TryFrom<&str> for DappParsedId {
    type Error = String;

    #[inline]
    fn try_from(dapp_id: &str) -> Result<Self, Self::Error> {
        if !dapp_id.starts_with("in") {
            return Err("wrong dapp id".into());
        }

        std::panic::catch_unwind(|| {
            let data = bs58::decode(dapp_id.split_at(2).1)
                .into_vec()
                .map_err(|_| "wrong dapp id".to_string())?;

            // validate hash
            if data.len() < 4 {
                return Err("wrong dapp id".into());
            }
            let hash = hash_sha256(&hex::encode(&data[..data.len() - 4]));
            if data[data.len() - 4..] != hash[..4] {
                return Err("wrong dapp id".into());
            }
            let data = &data[..data.len() - 4];

            // First byte marker length
            let length = ((data[0] >> 4) & 0x0F) as usize; // CANISTER ID byte length
            let len = (data[0] & 0x0F) as usize; // Canister ID removes the length of the previous 0

            if data.len() < len as usize + 1 {
                return Err("wrong dapp id".into());
            }

            let mut canister_id = vec![0; length];
            for i in 0..len {
                let target = i + length - len;
                if canister_id.len() <= target {
                    return Err("wrong dapp id".to_string());
                }
                canister_id[target] = *data.get(i + 1).ok_or_else(|| "wrong dapp id".to_string())?;
            }
            let canister_id = Principal::from_slice(&canister_id);

            let (nonce, used) = bytes_to_nonce(&data[len + 1..]);

            let mut id = [0; 8];
            for i in 0..(data.len() - len - 1 - used) {
                id[7 - i] = data[data.len() - 1 - i];
            }

            let id = u64::from_be_bytes(id).into();

            Ok(Self { canister_id, id, nonce })
        })
        .map_err(|_| "wrong dapp id".to_string())?
    }
}

impl From<&DappParsedId> for DappAnchor {
    fn from(value: &DappParsedId) -> Self {
        // to_be_bytes The large end method indicates that 0 in front of the data can be saved
        // to_le_bytes The small end method indicates that the at the end of the data can be saved
        let canister_id = value.canister_id.as_slice();
        let nonce_bytes = nonce_to_bytes(value.nonce);
        let id_bytes = value.id.as_ref().to_be_bytes();

        let length = canister_id.len() as u8; // CANISTER ID byte length
        let canister_id = canister_id.iter().skip_while(|b| **b == 0).cloned().collect::<Vec<_>>();
        let len = canister_id.len() as u8; // Canister ID removes the length of the previous 0

        // println!("{:?} {length} {len} {} {}", canister_id, length << 4, length << 4 | len);

        let mut data = Vec::new();
        data.push((length << 4) | len);
        data.extend(canister_id);

        data.extend(nonce_bytes);

        data.extend(id_bytes.into_iter().skip_while(|b| *b == 0));

        // println!("{:?}", data);

        let hash = hash_sha256(&hex::encode(&data));
        data.extend_from_slice(&hash[0..4]); // The first four after the Hash as a verification

        Self::from(format!("{PREFIX}{}", bs58::encode(data).into_string()))
    }
}

impl DappParsedId {
    /// new
    pub fn from(canister_id: Principal, id: DappId, nonce: Option<u32>) -> Self {
        Self { canister_id, id, nonce }
    }

    /// check
    pub fn check_canister_id(&self, self_canister_id: &Principal) -> Result<(), String> {
        if self.canister_id != *self_canister_id {
            return Err("canister id mismatch".into());
        }
        Ok(())
    }
}

/// Prefix method conversion number
fn nonce_to_bytes(nonce: Option<u32>) -> Vec<u8> {
    let mut nonce = nonce.unwrap_or_default();

    let mut bytes = vec![];

    while 0 < nonce {
        let mut byte = (nonce & 0b0111_1111) as u8;
        nonce >>= 7;
        if nonce != 0 {
            byte |= 0b1000_0000;
        }
        bytes.push(byte);
    }

    if bytes.is_empty() {
        bytes.push(0); // At least 1 byte
    }

    bytes
}

fn bytes_to_nonce(bytes: &[u8]) -> (Option<u32>, usize) {
    let mut nonce: u32 = 0;
    let mut i = 0; // Number of numbers used statistical

    for byte in bytes.iter() {
        nonce |= ((byte & 0b0111_1111) as u32) << (i * 7);
        if byte & 0b1000_0000 == 0 {
            break;
        }
        i += 1;
    }

    (if 0 < nonce { Some(nonce) } else { None }, i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let id: DappAnchor = (&DappParsedId::from(
            Principal::from_text("fmzms-paaaa-aaaai-qpeka-cai").unwrap(),
            9123123123123123123.into(),
            None,
        ))
            .into();
        println!("{:?}", id);
        let id: DappParsedId = id.as_ref().as_str().try_into().unwrap();
        println!("{} {} {:?}", id.canister_id.to_text(), id.id.as_ref(), id.nonce);

        let id: DappAnchor = (&DappParsedId::from(
            Principal::from_text("fmzms-paaaa-aaaai-qpeka-cai").unwrap(),
            9123123123123123123.into(),
            Some(123),
        ))
            .into();
        println!("{:?}", id);
        let id: DappParsedId = id.as_ref().as_str().try_into().unwrap();
        println!("{} {} {:?}", id.canister_id.to_text(), id.id.as_ref(), id.nonce);

        let id: DappAnchor = (&DappParsedId::from(
            Principal::from_text("fmzms-paaaa-aaaai-qpeka-cai").unwrap(),
            9123123123123123123.into(),
            Some(11728),
        ))
            .into();
        println!("{:?}", id);
        let id: DappParsedId = id.as_ref().as_str().try_into().unwrap();
        println!("{} {} {:?}", id.canister_id.to_text(), id.id.as_ref(), id.nonce);
    }
}
