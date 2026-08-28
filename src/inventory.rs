use crate::catalog;
use crate::model::{
    Category, ChecklistItem, Inputs, Location, Report, Summary, Support, TargetApp,
    VerificationItem,
};
use crate::xmp;
use anyhow::{Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &[
    "3fr", "arw", "avif", "cr2", "cr3", "dng", "erf", "gif", "heic", "heif", "jpeg", "jpg", "mos",
    "mrw", "nef", "orf", "pef", "png", "raf", "raw", "rw2", "srw", "tif", "tiff", "webp",
];

#[derive(Clone, Debug)]
pub struct ScanOptions {
    pub source: PathBuf,
    pub target: PathBuf,
    pub catalog: Option<PathBuf>,
    pub target_app: TargetApp,
    pub sample_size: usize,
}

pub fn scan(options: &ScanOptions) -> Result<Report> {
    validate(options)?;
    let source_files = walk_library(&options.source)?;
    let target_files = walk_library(&options.target)?;
    let xmp_paths: Vec<PathBuf> = source_files
        .all_files
        .iter()
        .filter(|path| extension_lower(path).as_deref() == Some("xmp"))
        .cloned()
        .collect();
    let xmp = xmp::inspect(&xmp_paths, &options.source)?;
    let catalog = options
        .catalog
        .as_deref()
        .map(catalog::inspect)
        .transpose()?;

    let source_map = asset_map(&source_files.images, &options.source);
    let target_map = asset_map(&target_files.images, &options.target);
    let matches = match_assets(&source_map, &target_map);
    let matched_assets = matches.values().filter(|item| item.is_some()).count();

    let mut warnings = source_files.warnings;
    warnings.extend(target_files.warnings);
    warnings.extend(
        xmp.malformed
            .iter()
            .map(|item| format!("Skipped malformed XMP {item}")),
    );
    if xmp.sidecars == 0 {
        warnings.push("No XMP sidecars found; catalog-only risk may be understated.".to_owned());
    }
    if let Some(catalog) = &catalog {
        if !catalog.recognized {
            warnings.push(format!(
                "The SQLite file has {} tables but no recognized Lightroom core tables.",
                catalog.tables
            ));
        }
    } else {
        warnings.push(
            "No Lightroom catalog supplied; catalog-only fields cannot be discovered.".to_owned(),
        );
    }

    let categories = build_categories(
        source_map.len(),
        &xmp.counts,
        catalog.as_ref().map(|item| &item.counts),
        options.target_app,
    );
    let catalog_only_fields = categories
        .iter()
        .filter(|item| item.location == Location::CatalogOnly && item.records > 0)
        .count();
    let target_unsupported_fields = categories
        .iter()
        .filter(|item| item.target_support == Support::Unsupported && item.records > 0)
        .count();

    let summary = Summary {
        source_assets: source_map.len(),
        target_assets: target_map.len(),
        matched_assets,
        missing_assets: source_map.len().saturating_sub(matched_assets),
        xmp_sidecars: xmp.sidecars,
        catalog_records: catalog.as_ref().map_or(0, |item| item.records),
        catalog_only_fields,
        target_unsupported_fields,
    };
    let checklist = build_checklist(&summary, &categories, &warnings, options.target_app);
    let verification_sample = build_sample(&matches, options.sample_size, &categories);

    Ok(Report {
        schema_version: "1.0".to_owned(),
        generated_at_unix: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        target_app: options.target_app,
        inputs: Inputs {
            source: canonical_or_original(&options.source),
            target: canonical_or_original(&options.target),
            catalog: options.catalog.as_deref().map(canonical_or_original),
        },
        summary,
        categories,
        checklist,
        verification_sample,
        warnings,
    })
}

fn validate(options: &ScanOptions) -> Result<()> {
    for (label, path) in [("source", &options.source), ("target", &options.target)] {
        if !path.exists() {
            bail!("{label} folder does not exist: {}", path.display());
        }
        if !path.is_dir() {
            bail!("{label} must be a folder: {}", path.display());
        }
    }
    if options.source == options.target {
        bail!("source and target must be different folders");
    }
    if let Some(path) = &options.catalog {
        if !path.is_file() {
            bail!(
                "catalog does not exist or is not a file: {}",
                path.display()
            );
        }
    }
    if options.sample_size > 100 {
        bail!("sample size cannot exceed 100");
    }
    Ok(())
}

#[derive(Default)]
struct LibraryWalk {
    all_files: Vec<PathBuf>,
    images: Vec<PathBuf>,
    warnings: Vec<String>,
}

fn walk_library(root: &Path) -> Result<LibraryWalk> {
    let mut result = LibraryWalk::default();
    for entry in WalkDir::new(root).follow_links(false).sort_by_file_name() {
        match entry {
            Ok(entry) if entry.file_type().is_file() => {
                let path = entry.into_path();
                if is_image(&path) {
                    result.images.push(path.clone());
                }
                result.all_files.push(path);
            }
            Ok(_) => {}
            Err(error) => result
                .warnings
                .push(format!("Could not inspect a path: {error}")),
        }
    }
    Ok(result)
}

fn is_image(path: &Path) -> bool {
    extension_lower(path).is_some_and(|ext| IMAGE_EXTENSIONS.binary_search(&ext.as_str()).is_ok())
}

fn extension_lower(path: &Path) -> Option<String> {
    Some(path.extension()?.to_str()?.to_ascii_lowercase())
}

fn asset_map(paths: &[PathBuf], root: &Path) -> BTreeMap<String, String> {
    paths
        .iter()
        .filter_map(|path| {
            let relative = path.strip_prefix(root).ok()?;
            let mut without_extension = relative.to_path_buf();
            without_extension.set_extension("");
            let key = normalized_path(&without_extension);
            Some((key, normalized_path(relative)))
        })
        .collect()
}

fn normalized_path(path: &Path) -> String {
    path.components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn match_assets(
    sources: &BTreeMap<String, String>,
    targets: &BTreeMap<String, String>,
) -> BTreeMap<String, Option<String>> {
    let mut by_name: BTreeMap<String, Vec<&String>> = BTreeMap::new();
    for (key, value) in targets {
        let name = key.rsplit('/').next().unwrap_or(key).to_ascii_lowercase();
        by_name.entry(name).or_default().push(value);
    }
    sources
        .iter()
        .map(|(key, source)| {
            let exact = targets.get(key).cloned();
            let fallback = key
                .rsplit('/')
                .next()
                .and_then(|name| by_name.get(&name.to_ascii_lowercase()))
                .and_then(|items| (items.len() == 1).then(|| (*items[0]).clone()));
            (source.clone(), exact.or(fallback))
        })
        .collect()
}

fn build_categories(
    source_count: usize,
    xmp: &BTreeMap<String, usize>,
    catalog: Option<&BTreeMap<String, usize>>,
    target: TargetApp,
) -> Vec<Category> {
    let fields = [
        ("camera_exif", "Camera and lens EXIF"),
        ("orientation", "Orientation"),
        ("capture_date", "Corrected capture date"),
        ("rating", "Star rating"),
        ("color_label", "Color label"),
        ("pick_flags", "Pick and reject flags"),
        ("keywords", "Keywords and hierarchy"),
        ("title", "Title"),
        ("caption", "Caption"),
        ("copyright", "Copyright"),
        ("location", "GPS and location"),
        ("people", "People and face regions"),
        ("collections", "Collections"),
        ("stacks", "Stacks"),
        ("virtual_copies", "Virtual copies"),
        ("develop_recipe", "Lightroom develop recipe"),
        ("develop_history", "Develop history"),
    ];

    fields
        .into_iter()
        .filter_map(|(field, label)| {
            let xmp_count = xmp.get(field).copied().unwrap_or(0);
            let catalog_count = catalog
                .and_then(|counts| counts.get(field))
                .copied()
                .unwrap_or(0);
            let (location, records) = if matches!(field, "camera_exif" | "orientation")
                && xmp_count == 0
                && catalog_count == 0
                && source_count > 0
            {
                (Location::Embedded, source_count)
            } else if xmp_count > 0 {
                (Location::Sidecar, xmp_count)
            } else if catalog_count > 0 {
                (Location::CatalogOnly, catalog_count)
            } else {
                return None;
            };
            let target_support = support_for(target, field, location);
            let detail = detail_for(field, location, xmp_count, catalog_count);
            let action = action_for(field, location, target_support);
            Some(Category {
                field: field.to_owned(),
                label: label.to_owned(),
                location,
                target_support,
                records,
                detail,
                action,
            })
        })
        .collect()
}

fn support_for(target: TargetApp, field: &str, location: Location) -> Support {
    if matches!(
        field,
        "develop_recipe" | "develop_history" | "virtual_copies"
    ) {
        return Support::Unsupported;
    }
    if matches!(field, "collections" | "stacks" | "pick_flags" | "people") {
        return match target {
            TargetApp::Digikam if matches!(field, "people" | "pick_flags") => Support::Partial,
            TargetApp::Darktable if field == "pick_flags" => Support::Partial,
            _ => Support::Unsupported,
        };
    }
    match target {
        TargetApp::Generic => {
            if location == Location::Embedded {
                Support::Supported
            } else {
                Support::Unknown
            }
        }
        TargetApp::Immich => match field {
            "camera_exif" | "orientation" | "capture_date" | "keywords" | "caption" | "title"
            | "copyright" | "location" => Support::Supported,
            "rating" | "color_label" => Support::Partial,
            _ => Support::Unknown,
        },
        TargetApp::Darktable => match field {
            "camera_exif" | "orientation" | "capture_date" | "rating" | "color_label"
            | "keywords" | "caption" | "title" | "copyright" | "location" => Support::Supported,
            _ => Support::Unknown,
        },
        TargetApp::Digikam => match field {
            "camera_exif" | "orientation" | "capture_date" | "rating" | "color_label"
            | "keywords" | "caption" | "title" | "copyright" | "location" => Support::Supported,
            _ => Support::Unknown,
        },
    }
}

fn detail_for(field: &str, location: Location, xmp_count: usize, catalog_count: usize) -> String {
    if field == "camera_exif" {
        return "Expected in image metadata; binary image contents were not opened.".to_owned();
    }
    let mut detail = match location {
        Location::Embedded => "Expected inside each image container.".to_owned(),
        Location::Sidecar => format!("Observed in {xmp_count} XMP sidecar(s)."),
        Location::CatalogOnly => format!(
            "Observed in up to {catalog_count} catalog record(s), but not in any XMP sidecar."
        ),
    };
    if xmp_count > 0 && catalog_count > 0 {
        detail.push_str(&format!(
            " Also present in up to {catalog_count} catalog record(s)."
        ));
    }
    detail
}

fn action_for(field: &str, location: Location, support: Support) -> String {
    if matches!(field, "develop_recipe" | "develop_history") {
        "Render a finished derivative for critical edits and keep the original catalog.".to_owned()
    } else if field == "virtual_copies" {
        "Render or duplicate each wanted version before migration.".to_owned()
    } else if location == Location::CatalogOnly {
        "Write metadata to XMP from Lightroom or document it before moving.".to_owned()
    } else if support == Support::Unsupported {
        "Export a neutral record and verify a manual recreation in the destination.".to_owned()
    } else if support == Support::Partial || support == Support::Unknown {
        "Check this field on the verification sample in the destination.".to_owned()
    } else {
        "Verify one old, one recent, and one edited file after import.".to_owned()
    }
}

fn build_checklist(
    summary: &Summary,
    categories: &[Category],
    warnings: &[String],
    target: TargetApp,
) -> Vec<ChecklistItem> {
    let mut items = Vec::new();
    if summary.missing_assets > 0 {
        items.push(ChecklistItem {
            priority: "blocker".to_owned(),
            task: format!(
                "Copy or account for {} missing source asset(s).",
                summary.missing_assets
            ),
            why: "A metadata-perfect migration is still incomplete when originals are absent."
                .to_owned(),
        });
    }
    let catalog_labels: Vec<&str> = categories
        .iter()
        .filter(|item| item.location == Location::CatalogOnly)
        .map(|item| item.label.as_str())
        .collect();
    if !catalog_labels.is_empty() {
        items.push(ChecklistItem {
            priority: "blocker".to_owned(),
            task: format!(
                "Export or record catalog-only state: {}.",
                catalog_labels.join(", ")
            ),
            why: "These values were found in the catalog but not in an XMP sidecar.".to_owned(),
        });
    }
    let unsupported: Vec<&str> = categories
        .iter()
        .filter(|item| item.target_support == Support::Unsupported)
        .map(|item| item.label.as_str())
        .collect();
    if !unsupported.is_empty() {
        items.push(ChecklistItem {
            priority: "blocker".to_owned(),
            task: format!(
                "Plan alternatives for {} in {target}: {}.",
                unsupported.len(),
                unsupported.join(", ")
            ),
            why: "The selected destination cannot faithfully consume these representations."
                .to_owned(),
        });
    }
    items.push(ChecklistItem {
        priority: "verify".to_owned(),
        task: "Import the verification sample before the full library.".to_owned(),
        why: "Target versions and settings vary; direct inspection is the final authority."
            .to_owned(),
    });
    items.push(ChecklistItem {
        priority: "safety".to_owned(),
        task: "Keep originals, XMP files, and a catalog backup until verification is complete."
            .to_owned(),
        why: "This report is read-only and does not create a recoverable migration backup."
            .to_owned(),
    });
    if !warnings.is_empty() {
        items.push(ChecklistItem {
            priority: "review".to_owned(),
            task: format!("Review {} scan warning(s).", warnings.len()),
            why: "Unreadable or omitted inputs reduce inventory coverage.".to_owned(),
        });
    }
    items
}

fn build_sample(
    matches: &BTreeMap<String, Option<String>>,
    sample_size: usize,
    categories: &[Category],
) -> Vec<VerificationItem> {
    let checks: Vec<String> = categories
        .iter()
        .filter(|item| item.records > 0)
        .take(6)
        .map(|item| item.label.clone())
        .collect();
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();
    for (source, target) in matches.iter().filter(|(_, target)| target.is_none()) {
        if selected.len() >= sample_size {
            break;
        }
        seen.insert(source.clone());
        selected.push((source, target));
    }
    if selected.len() < sample_size && !matches.is_empty() {
        let remaining = sample_size - selected.len();
        let step = (matches.len() / remaining.max(1)).max(1);
        for (source, target) in matches.iter().step_by(step) {
            if selected.len() >= sample_size {
                break;
            }
            if seen.insert(source.clone()) {
                selected.push((source, target));
            }
        }
    }
    selected
        .into_iter()
        .map(|(source, target)| VerificationItem {
            source: source.clone(),
            target: target.clone(),
            status: if target.is_some() {
                "matched"
            } else {
                "missing"
            }
            .to_owned(),
            verify: checks.clone(),
        })
        .collect()
}

fn canonical_or_original(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::{ScanOptions, is_image, scan};
    use crate::model::{Location, TargetApp};
    use rusqlite::Connection;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn image_extensions_are_recognized() {
        assert!(is_image(std::path::Path::new("frame.cr3")));
        assert!(!is_image(std::path::Path::new("notes.txt")));
    }

    #[test]
    fn scans_xmp_catalog_and_target_without_writes() {
        let temp = tempdir().unwrap();
        let source = temp.path().join("source");
        let target = temp.path().join("target");
        fs::create_dir_all(&source).unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::write(source.join("one.CR3"), b"not opened").unwrap();
        fs::write(source.join("two.jpg"), b"not opened").unwrap();
        fs::write(target.join("one.jpg"), b"not opened").unwrap();
        fs::write(
            source.join("one.xmp"),
            r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="5" /></rdf:RDF></x:xmpmeta>"#,
        )
        .unwrap();
        let catalog = temp.path().join("library.lrcat");
        let db = Connection::open(&catalog).unwrap();
        db.execute_batch(
            "CREATE TABLE Adobe_images (id_local INTEGER, rating INTEGER, pick INTEGER, developSettingsID INTEGER); INSERT INTO Adobe_images VALUES (1,5,1,7),(2,0,0,8);",
        )
        .unwrap();
        drop(db);

        let report = scan(&ScanOptions {
            source,
            target,
            catalog: Some(catalog),
            target_app: TargetApp::Immich,
            sample_size: 2,
        })
        .unwrap();
        assert_eq!(report.summary.source_assets, 2);
        assert_eq!(report.summary.matched_assets, 1);
        assert_eq!(report.summary.missing_assets, 1);
        assert_eq!(report.summary.xmp_sidecars, 1);
        assert!(
            report.categories.iter().any(|item| {
                item.field == "pick_flags" && item.location == Location::CatalogOnly
            })
        );
        assert_eq!(report.verification_sample[0].status, "missing");
    }
}
