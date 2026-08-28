use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum TargetApp {
    Generic,
    Immich,
    Darktable,
    Digikam,
}

impl fmt::Display for TargetApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Generic => "generic",
            Self::Immich => "immich",
            Self::Darktable => "darktable",
            Self::Digikam => "digikam",
        };
        f.write_str(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Location {
    Embedded,
    Sidecar,
    CatalogOnly,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Embedded => "embedded",
            Self::Sidecar => "sidecar",
            Self::CatalogOnly => "catalog-only",
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Support {
    Supported,
    Partial,
    Unsupported,
    Unknown,
}

impl fmt::Display for Support {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Supported => "supported",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::Unknown => "verify",
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inputs {
    pub source: PathBuf,
    pub target: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<PathBuf>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Summary {
    pub source_assets: usize,
    pub target_assets: usize,
    pub matched_assets: usize,
    pub missing_assets: usize,
    pub xmp_sidecars: usize,
    pub catalog_records: usize,
    pub catalog_only_fields: usize,
    pub target_unsupported_fields: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub field: String,
    pub label: String,
    pub location: Location,
    pub target_support: Support,
    pub records: usize,
    pub detail: String,
    pub action: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationItem {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub status: String,
    pub verify: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChecklistItem {
    pub priority: String,
    pub task: String,
    pub why: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Report {
    pub schema_version: String,
    pub generated_at_unix: u64,
    pub target_app: TargetApp,
    pub inputs: Inputs,
    pub summary: Summary,
    pub categories: Vec<Category>,
    pub checklist: Vec<ChecklistItem>,
    pub verification_sample: Vec<VerificationItem>,
    pub warnings: Vec<String>,
}

impl Report {
    pub fn has_blockers(&self) -> bool {
        self.summary.missing_assets > 0
            || self.summary.catalog_only_fields > 0
            || self.summary.target_unsupported_fields > 0
    }
}
