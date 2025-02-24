use array::ViewArrayMetadata;
use bool::ViewBoolMetadata;
use html::ViewHtmlMetadata;
use image::ViewImageMetadata;
use object::ViewObjectMetadata;
use serde::{Deserialize, Serialize};
use table::ViewTableMetadata;
use text::ViewTextMetadata;

use super::{AllEndpoints, CodeValue, ComponentId, Endpoint, InputValue, LinkError, LinkType, LinkValue};

/// view inner
pub mod inner;

/// view text
pub mod text;

/// view bool
pub mod bool;

/// view image
pub mod image;

/// view table
pub mod table;

/// view html
pub mod html;

/// view array
pub mod array;

/// view object
pub mod object;

use inner::{
    InnerViewArrayMetadata, InnerViewBoolMetadata, InnerViewHtmlMetadata, InnerViewImageMetadata, InnerViewMetadata,
    InnerViewObjectItem, InnerViewObjectMetadata, InnerViewTableMetadata, InnerViewTextMetadata,
};

/// identity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentView {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub inlets: Option<Vec<Endpoint>>,

    /// Metadata required for this component execution
    pub metadata: ViewMetadata,
    // Output type // There is no output type in the display component
    // pub output: LinkType,
}

/// view metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ViewMetadata {
    /// text
    #[serde(rename = "text")]
    Text(ViewTextMetadata),
    /// bool
    #[serde(rename = "bool")]
    Bool(ViewBoolMetadata),
    /// image
    #[serde(rename = "image")]
    Image(ViewImageMetadata),
    /// table
    #[serde(rename = "table")]
    Table(ViewTableMetadata),
    /// html
    #[serde(rename = "html")]
    Html(ViewHtmlMetadata),

    // Temporarily support 2 combination display
    /// array
    #[serde(rename = "array")]
    Array(ViewArrayMetadata),
    /// object
    #[serde(rename = "object")]
    Object(ViewObjectMetadata),
}

impl ComponentView {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// Check whether the component is effective
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>) -> Result<Self, LinkError> {
        // 1 Check metadata
        let metadata = match &self.metadata {
            ViewMetadata::Text(text) => ViewMetadata::Text(text.check(endpoints, self.id)?),
            ViewMetadata::Bool(bool) => ViewMetadata::Bool(bool.check(endpoints, self.id)?),
            ViewMetadata::Image(image) => ViewMetadata::Image(image.check(endpoints, self.id)?),
            ViewMetadata::Table(table) => ViewMetadata::Table(table.check(endpoints, self.id)?),
            ViewMetadata::Html(html) => ViewMetadata::Html(html.check(endpoints, self.id)?),
            ViewMetadata::Array(array) => ViewMetadata::Array(array.check(endpoints, self.id)?),
            ViewMetadata::Object(object) => ViewMetadata::Object(object.check(endpoints, self.id)?),
        };

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata,
        })
    }
}
