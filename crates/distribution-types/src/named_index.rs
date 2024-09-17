use url::Url;

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct IndexSource {
    pub name: String,
    pub index: Url,
    #[serde(default)]
    pub kind: IndexKind,
}

#[derive(
    Default, Debug, Copy, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum IndexKind {
    /// A PEP 503 and/or PEP 691-compliant index.
    #[default]
    Simple,
    /// An index containing a list of links to distributions (e.g., `--find-links`).
    Flat,
}
