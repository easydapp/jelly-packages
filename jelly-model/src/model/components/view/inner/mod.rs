use serde::{Deserialize, Serialize};

use super::{ComponentId, LinkError, LinkType};

/// text
pub mod text;

/// bool
pub mod bool;

/// image
pub mod image;

/// table
pub mod table;

/// html
pub mod html;

/// array
pub mod array;

/// object
pub mod object;

pub(super) use text::InnerViewTextMetadata;

pub(super) use bool::InnerViewBoolMetadata;

pub(super) use image::InnerViewImageMetadata;

pub(super) use table::InnerViewTableMetadata;

pub(super) use html::InnerViewHtmlMetadata;

pub(super) use array::InnerViewArrayMetadata;

pub(super) use object::{InnerViewObjectItem, InnerViewObjectMetadata};

/// view metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum InnerViewMetadata {
    /// text
    #[serde(rename = "text")]
    Text(InnerViewTextMetadata),
    /// bool
    #[serde(rename = "bool")]
    Bool(InnerViewBoolMetadata),
    /// image
    #[serde(rename = "image")]
    Image(InnerViewImageMetadata),
    /// table
    #[serde(rename = "table")]
    Table(InnerViewTableMetadata),
    /// html
    #[serde(rename = "html")]
    Html(InnerViewHtmlMetadata),

    // Temporarily support 2 combination display
    /// array
    #[serde(rename = "array")]
    Array(InnerViewArrayMetadata),
    /// object
    #[serde(rename = "object")]
    Object(InnerViewObjectMetadata),
}

impl InnerViewMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        match self {
            InnerViewMetadata::Text(_) => InnerViewTextMetadata::is_supported_type(ty),
            InnerViewMetadata::Bool(_) => InnerViewBoolMetadata::is_supported_type(ty),
            InnerViewMetadata::Image(_) => InnerViewImageMetadata::is_supported_type(ty),
            InnerViewMetadata::Table(_) => InnerViewTableMetadata::is_supported_type(ty),
            InnerViewMetadata::Html(html) => InnerViewHtmlMetadata::is_supported_type(&html.template, ty),
            InnerViewMetadata::Array(array) => InnerViewArrayMetadata::is_supported_type(&array.inner, ty),
            InnerViewMetadata::Object(object) => InnerViewObjectMetadata::is_supported_type(&object.inner, ty),
        }
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        match self {
            InnerViewMetadata::Text(text) => text.check(),
            InnerViewMetadata::Bool(bool) => bool.check(),
            InnerViewMetadata::Image(image) => image.check(),
            InnerViewMetadata::Table(table) => table.check(),
            InnerViewMetadata::Html(html) => html.check(from),
            InnerViewMetadata::Array(array) => array.check(from),
            InnerViewMetadata::Object(object) => object.check(from),
        }
    }
}
