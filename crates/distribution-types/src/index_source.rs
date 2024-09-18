use crate::IndexUrl;

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct IndexSource {
    /// The name of the index.
    ///
    /// Index names can be used to reference indexes elsewhere in the configuration. For example,
    /// you can pin a package to a specific index by name:
    ///
    /// ```toml
    /// [[tool.uv.index]]
    /// name = "pytorch"
    /// url = "https://download.pytorch.org/whl/cu121"
    ///
    /// [tool.uv.sources]
    /// torch = { index = "pytorch" }
    /// ```
    pub name: Option<String>,
    /// The URL of the index.
    ///
    /// Expects to receive a URL (e.g., `https://pypi.org/simple`) or a local path.
    pub url: IndexUrl,
    /// Mark the index as explicit.
    ///
    /// Explicit indexes will _only_ be used when explicitly enabled via a `[tool.uv.sources]`
    /// definition, as in:
    ///
    /// ```toml
    /// [[tool.uv.index]]
    /// name = "pytorch"
    /// url = "https://download.pytorch.org/whl/cu121"
    /// explicit = true
    ///
    /// [tool.uv.sources]
    /// torch = { index = "pytorch" }
    /// ```
    #[serde(default)]
    pub explicit: bool,
    /// Mark the index as the default index.
    ///
    /// By default, uv uses PyPI as the default index, such that even if additional indexes are
    /// defined via `[[tool.uv.index]]`, PyPI will still be used as a fallback for packages that
    /// aren't found elsewhere. To disable the PyPI default, set `default = true` on at least one
    /// other index.
    ///
    /// Marking an index as default will move it to the front of the list of indexes, such that it
    /// is given the highest priority when resolving packages.
    #[serde(default)]
    pub default: bool,
    // /// The type of the index.
    // ///
    // /// Indexes can either be PEP 503-compliant (i.e., a registry implementing the Simple API) or
    // /// structured as a flat list of distributions (e.g., `--find-links`). In both cases, indexes
    // /// can point to either local or remote resources.
    // #[serde(default)]
    // pub r#type: IndexKind,
}

// #[derive(
//     Default, Debug, Copy, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize,
// )]
// #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
// pub enum IndexKind {
//     /// A PEP 503 and/or PEP 691-compliant index.
//     #[default]
//     Simple,
//     /// An index containing a list of links to distributions (e.g., `--find-links`).
//     Flat,
// }
