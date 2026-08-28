//! Read-only inventory engine for photo-library migration pre-flight checks.

mod catalog;
mod inventory;
pub mod license;
mod model;
mod render;
mod xmp;

pub use inventory::{ScanOptions, scan};
pub use model::{Category, ChecklistItem, Location, Report, Support, TargetApp};
pub use render::render_text;
