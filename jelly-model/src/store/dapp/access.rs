use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

use crate::types::TimestampMills;

/// duration
pub mod duration;

/// chain
pub mod chain;

/// chain identity
pub mod chain_identity;

/// token balance
pub mod token_balance;

/// nft owner
pub mod nft_owner;

/// Single access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappAccessItem {
    /// Restriction visit time
    #[serde(rename = "duration")]
    Duration(duration::AccessDuration),
    /// Limit the maximum number of visits // ! Don’t know how to achieve it for the time being
    #[serde(rename = "times")]
    Times(u64),
    /// Restricted tokens
    #[serde(rename = "token")]
    Token(String),
    /// You can access the specified identity before accessing // ! Don’t know how to achieve it for the time being
    #[serde(rename = "chain_identity")]
    ChainIdentity(chain_identity::ChainIdentity),
    /// Limited to own the amount of tokens to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "token_balance")]
    TokenBalance(token_balance::TokenBalance),
    /// Limited to have NFT to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "nft_owner")]
    NftOwner(nft_owner::NFTOwner),
}

impl DappAccessItem {
    /// Simple access request
    pub fn access_by_timestamp_and_token(&self, now: TimestampMills, verified: &DappVerifiedItem) -> Option<bool> {
        match (self, verified) {
            (DappAccessItem::Duration(duration), DappVerifiedItem::Duration(verified)) => {
                if !duration.is_same(verified) {
                    return None;
                }
                Some(duration.access(now))
            }
            (DappAccessItem::Times(_), DappVerifiedItem::Times) => None, // ! Don’t know how to achieve it for the time being
            (DappAccessItem::Token(token), DappVerifiedItem::Token(verified)) => Some(token == verified),
            (DappAccessItem::ChainIdentity(_), DappVerifiedItem::ChainIdentity(_)) => None, // ! Don’t know how to achieve it for the time being
            (DappAccessItem::TokenBalance(_), DappVerifiedItem::TokenBalance(_)) => None, // ! Don’t know how to achieve it for the time being
            (DappAccessItem::NftOwner(_), DappVerifiedItem::NftOwner(_)) => None, // ! Don’t know how to achieve it for the time being
            _ => None, // ! The type is not right, it is wrong
        }
    }
}

/// Access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappAccess {
    /// Unlimited access
    #[serde(rename = "none")]
    None,
    /// Complete restrictions on access, you can only access on the official website through Admin
    #[serde(rename = "exclusive")]
    Exclusive,
    /// Must be satisfied
    #[serde(rename = "required")]
    Required(DappAccessItem),
    /// Must be dissatisfied
    #[serde(rename = "deny")]
    Deny(DappAccessItem),
    /// Must be satisfied
    #[serde(rename = "all")]
    All(Vec<DappAccess>),
    /// Satisfy any one
    #[serde(rename = "any")]
    Any(Vec<DappAccess>),
    /// If you satisfy, refuse
    #[serde(rename = "not")]
    Not(Vec<DappAccess>),
}

impl Storable for DappAccess {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        #[allow(clippy::unwrap_used)] // ? SAFETY
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        #[allow(clippy::expect_used)] // ? SAFETY
        ciborium::de::from_reader(&bytes[..]).expect("deserialization must succeed.")
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

impl DappAccess {
    fn inner_access_by_timestamp_and_token(
        &self,
        now: TimestampMills,
        verified: Option<&DappVerified>,
    ) -> Option<bool> {
        // No permission, all successful
        if matches!(self, DappAccess::None) {
            return Some(true);
        }

        // Complete restrictions on access, all failure
        if matches!(self, DappAccess::Exclusive) {
            return Some(false);
        }

        // Remove the verification
        let verified = match verified {
            Some(verified) => verified,
            None => return Some(false),
        };

        match (self, verified) {
            (DappAccess::Required(item), DappVerified::Required(verified)) => {
                item.access_by_timestamp_and_token(now, verified)
            }
            (DappAccess::Deny(item), DappVerified::Deny(verified)) => {
                item.access_by_timestamp_and_token(now, verified).map(|v| !v)
            }
            (DappAccess::All(items), DappVerified::All(verified)) => {
                'OUTER: for item in items.iter() {
                    for v in verified.iter() {
                        if item
                            .inner_access_by_timestamp_and_token(now, Some(v))
                            .is_some_and(|v| v)
                        {
                            continue 'OUTER;
                        }
                    }
                    return Some(false);
                }
                Some(true)
            }
            (DappAccess::Any(items), DappVerified::Any(verified)) => {
                for item in items.iter() {
                    for v in verified.iter() {
                        if item
                            .inner_access_by_timestamp_and_token(now, Some(v))
                            .is_some_and(|v| v)
                        {
                            return Some(true);
                        }
                    }
                }
                Some(false)
            }
            (DappAccess::Not(items), DappVerified::Not(verified)) => {
                for item in items.iter() {
                    for v in verified.iter() {
                        if item
                            .inner_access_by_timestamp_and_token(now, Some(v))
                            .is_some_and(|v| v)
                        {
                            return Some(false);
                        }
                    }
                }
                Some(true)
            }
            _ => None, // ! The type is not right, it is wrong
        }
    }

    /// Simple access request
    pub fn access_by_timestamp_and_token(&self, now: TimestampMills, verified: Option<&DappVerified>) -> bool {
        self.inner_access_by_timestamp_and_token(now, verified)
            .is_some_and(|v| v)
    }
}

// ================== view ==================

/// Single access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappAccessItemView {
    /// Restriction visit time
    #[serde(rename = "duration")]
    Duration(duration::AccessDuration),
    /// Limit the maximum number of visits // ! Don’t know how to achieve it for the time being
    #[serde(rename = "times")]
    Times(u64),
    /// Restricted tokens
    #[serde(rename = "token")]
    Token,
    /// You can access the specified identity before accessing // ! Don’t know how to achieve it for the time being
    #[serde(rename = "chain_identity")]
    ChainIdentity(chain_identity::ChainIdentity),
    /// Limited to own the amount of tokens to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "token_balance")]
    TokenBalance(token_balance::TokenBalance),
    /// Limited to have NFT to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "nft_owner")]
    NftOwner(nft_owner::NFTOwner),
}

/// Access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappAccessView {
    /// Unlimited access
    #[serde(rename = "none")]
    None,
    /// Complete restrictions on access, you can only access on the official website through Admin
    #[serde(rename = "exclusive")]
    Exclusive,
    /// Must be satisfied
    #[serde(rename = "required")]
    Required(DappAccessItemView),
    /// Must be dissatisfied
    #[serde(rename = "deny")]
    Deny(DappAccessItemView),
    /// Must be satisfied
    #[serde(rename = "all")]
    All(Vec<DappAccessView>),
    /// Satisfy any one
    #[serde(rename = "any")]
    Any(Vec<DappAccessView>),
    /// If you satisfy, refuse
    #[serde(rename = "not")]
    Not(Vec<DappAccessView>),
}

impl From<DappAccessItem> for DappAccessItemView {
    fn from(value: DappAccessItem) -> Self {
        match value {
            DappAccessItem::Duration(duration) => DappAccessItemView::Duration(duration),
            DappAccessItem::Times(times) => DappAccessItemView::Times(times),
            DappAccessItem::Token(_) => DappAccessItemView::Token,
            DappAccessItem::ChainIdentity(chain_identity) => DappAccessItemView::ChainIdentity(chain_identity),
            DappAccessItem::TokenBalance(token_balance) => DappAccessItemView::TokenBalance(token_balance),
            DappAccessItem::NftOwner(nft_owner) => DappAccessItemView::NftOwner(nft_owner),
        }
    }
}

impl From<DappAccess> for DappAccessView {
    fn from(value: DappAccess) -> Self {
        match value {
            DappAccess::None => DappAccessView::None,
            DappAccess::Exclusive => DappAccessView::Exclusive,
            DappAccess::Required(required) => DappAccessView::Required(required.into()),
            DappAccess::Deny(deny) => DappAccessView::Deny(deny.into()),
            DappAccess::All(all) => DappAccessView::All(all.into_iter().map(|item| item.into()).collect()),
            DappAccess::Any(any) => DappAccessView::Any(any.into_iter().map(|item| item.into()).collect()),
            DappAccess::Not(not) => DappAccessView::Not(not.into_iter().map(|item| item.into()).collect()),
        }
    }
}

// ================== verified ==================

/// Single access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappVerifiedItem {
    /// Restriction visit time
    #[serde(rename = "duration")]
    Duration(duration::VerifiedAccessDuration),
    /// Limit the maximum number of visits // ! Don’t know how to achieve it for the time being
    #[serde(rename = "times")]
    Times, // No data is required
    /// Restricted tokens
    #[serde(rename = "token")]
    Token(String),
    /// You can access the specified identity before accessing // ! Don’t know how to achieve it for the time being
    #[serde(rename = "chain_identity")]
    ChainIdentity(chain_identity::VerifiedChainIdentity),
    /// Limited to own the amount of tokens to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "token_balance")]
    TokenBalance(token_balance::VerifiedTokenBalance),
    /// Limited to have NFT to access // ! Don’t know how to achieve it for the time being
    #[serde(rename = "nft_owner")]
    NftOwner(nft_owner::VerifiedNFTOwner),
}

/// Access permission
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappVerified {
    /// Unlimited access
    #[serde(rename = "none")]
    None,
    /// Complete restrictions on access, you can only access on the official website through Admin
    #[serde(rename = "exclusive")]
    Exclusive,
    /// Must be satisfied
    #[serde(rename = "required")]
    Required(DappVerifiedItem),
    /// Must be dissatisfied
    #[serde(rename = "deny")]
    Deny(DappVerifiedItem),
    /// Must be satisfied
    #[serde(rename = "all")]
    All(Vec<DappVerified>),
    /// Satisfy any one
    #[serde(rename = "any")]
    Any(Vec<DappVerified>),
    /// If you satisfy, refuse
    #[serde(rename = "not")]
    Not(Vec<DappVerified>),
}
