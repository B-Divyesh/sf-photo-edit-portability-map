use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use edit_portability_map::{ScanOptions, TargetApp, license, render_text, scan};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "edit-portability-map",
    version,
    about = "Inventory Lightroom metadata before moving a photo library",
    long_about = "Read-only scanner for Lightroom catalogs, XMP sidecars, and a target folder. It lists metadata that can move and metadata that needs checking."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inventory source metadata and compare source assets with a target folder
    Scan {
        /// Folder containing source originals and adjacent XMP sidecars
        #[arg(long, value_name = "DIR")]
        source: PathBuf,
        /// Destination/import folder to compare by relative stem and unique file name
        #[arg(long, value_name = "DIR")]
        target: PathBuf,
        /// Optional Lightroom .lrcat SQLite catalog, opened read-only
        #[arg(long, value_name = "FILE")]
        catalog: Option<PathBuf>,
        /// Destination capability profile
        #[arg(long, value_enum, default_value_t = TargetApp::Generic)]
        target_app: TargetApp,
        /// Print versioned JSON to stdout instead of the human report
        #[arg(long)]
        json: bool,
        /// Also save the human report to this file (outside source/target)
        #[arg(long, value_name = "FILE")]
        report: Option<PathBuf>,
        /// Also save versioned JSON to this file (outside source/target)
        #[arg(long, value_name = "FILE")]
        json_report: Option<PathBuf>,
        /// Number of deterministic files to include for destination verification
        #[arg(long, default_value_t = 10)]
        sample_size: usize,
        /// Exit with code 3 when missing, catalog-only, or unsupported state is found
        #[arg(long)]
        fail_on_blockers: bool,
    },
    /// Activate or inspect an optional local license token
    License {
        #[command(subcommand)]
        command: LicenseCommand,
    },
    /// Run the bundled sample scan in a new temporary folder
    Demo,
}

#[derive(Debug, Subcommand)]
enum LicenseCommand {
    /// Verify and store a license token in the user config directory
    Activate {
        /// License token returned by Sociobot checkout
        #[arg(value_name = "TOKEN")]
        token: String,
    },
    /// Show the locally cached license state without making a network request
    Status,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<u8> {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan {
            source,
            target,
            catalog,
            target_app,
            json,
            report,
            json_report,
            sample_size,
            fail_on_blockers,
        } => {
            if sample_size > 100 {
                bail!("sample size cannot exceed 100");
            }
            if sample_size > 10 {
                license::require_pro()?;
            }
            let resolved_inputs = ResolvedInputs::new(&source, &target, catalog.as_deref())?;
            for output in [&report, &json_report].into_iter().flatten() {
                ensure_output_is_safe(output, &resolved_inputs)?;
            }
            let report_data = scan(&ScanOptions {
                source,
                target,
                catalog,
                target_app,
                sample_size,
            })?;
            let text = render_text(&report_data);
            let json_text = serde_json::to_string_pretty(&report_data)?;
            if let Some(path) = report {
                fs::write(&path, &text)
                    .with_context(|| format!("could not write report {}", path.display()))?;
            }
            if let Some(path) = json_report {
                fs::write(&path, format!("{json_text}\n"))
                    .with_context(|| format!("could not write JSON report {}", path.display()))?;
            }
            if json {
                println!("{json_text}");
            } else {
                print!("{text}");
            }
            Ok(if fail_on_blockers && report_data.has_blockers() {
                3
            } else {
                0
            })
        }
        Command::License { command } => {
            match command {
                LicenseCommand::Activate { token } => {
                    license::activate(&token)?;
                    println!("Pro license verified and stored securely for daily checks.");
                }
                LicenseCommand::Status => println!("{}", license::status()?),
            }
            Ok(0)
        }
        Command::Demo => run_demo(),
    }
}

fn run_demo() -> Result<u8> {
    let root = tempfile::Builder::new()
        .prefix("edit-portability-map-demo-")
        .tempdir()?
        .keep();
    let source = root.join("Originals");
    let target = root.join("Target");
    let catalog = root.join("sample.lrcat");

    for (relative, contents) in [
        (
            "Originals/2024-04-12/harbor.CR3",
            include_bytes!("../examples/demo/Originals/2024-04-12/harbor.CR3").as_slice(),
        ),
        (
            "Originals/2024-04-12/harbor.xmp",
            include_bytes!("../examples/demo/Originals/2024-04-12/harbor.xmp").as_slice(),
        ),
        (
            "Originals/2024-04-12/studio.CR3",
            include_bytes!("../examples/demo/Originals/2024-04-12/studio.CR3").as_slice(),
        ),
        (
            "Originals/2024-04-12/studio.xmp",
            include_bytes!("../examples/demo/Originals/2024-04-12/studio.xmp").as_slice(),
        ),
        (
            "Originals/2024-04-13/rain.CR3",
            include_bytes!("../examples/demo/Originals/2024-04-13/rain.CR3").as_slice(),
        ),
        (
            "Target/2024-04-12/harbor.jpg",
            include_bytes!("../examples/demo/Target/2024-04-12/harbor.jpg").as_slice(),
        ),
        (
            "Target/2024-04-12/studio.jpg",
            include_bytes!("../examples/demo/Target/2024-04-12/studio.jpg").as_slice(),
        ),
        (
            "Target/2024-04-13/rain.jpg",
            include_bytes!("../examples/demo/Target/2024-04-13/rain.jpg").as_slice(),
        ),
    ] {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("demo files have parents"))?;
        fs::write(path, contents)?;
    }

    let connection = Connection::open(&catalog).with_context(|| {
        format!(
            "could not create bundled demo catalog {}",
            catalog.display()
        )
    })?;
    connection.execute_batch(include_str!("../examples/demo/catalog.sql"))?;
    drop(connection);

    let report = scan(&ScanOptions {
        source,
        target,
        catalog: Some(catalog),
        target_app: TargetApp::Immich,
        sample_size: 3,
    })?;
    let text = render_text(&report);
    fs::write(root.join("portability-report.txt"), &text)?;
    fs::write(
        root.join("portability-report.json"),
        format!("{}\n", serde_json::to_string_pretty(&report)?),
    )?;

    print!("{text}");
    println!("Demo sandbox: {}", root.display());
    println!("Reports: portability-report.txt and portability-report.json");
    println!("Only bundled files were created. Your photo library was not read or changed.");
    Ok(0)
}

struct ResolvedInputs {
    source: PathBuf,
    target: PathBuf,
    catalog: Option<PathBuf>,
}

impl ResolvedInputs {
    fn new(source: &Path, target: &Path, catalog: Option<&Path>) -> Result<Self> {
        let source = fs::canonicalize(source)
            .with_context(|| format!("could not resolve source {}", source.display()))?;
        let target = fs::canonicalize(target)
            .with_context(|| format!("could not resolve target {}", target.display()))?;
        if source == target {
            bail!("source and target must resolve to different folders");
        }
        Ok(Self {
            source,
            target,
            catalog: catalog
                .map(|path| {
                    fs::canonicalize(path)
                        .with_context(|| format!("could not resolve catalog {}", path.display()))
                })
                .transpose()?,
        })
    }
}

fn ensure_output_is_safe(output: &Path, inputs: &ResolvedInputs) -> Result<()> {
    let resolved_output = resolve_with_existing_ancestor(output)?;
    for (label, input) in [("source", &inputs.source), ("target", &inputs.target)] {
        if resolved_output.starts_with(input) {
            bail!(
                "refusing to write report inside the scanned {} folder: {}",
                label,
                output.display()
            );
        }
    }
    if let Some(catalog) = &inputs.catalog {
        let aliases_catalog = resolved_output == *catalog
            || (fs::symlink_metadata(output).is_ok()
                && same_file::is_same_file(output, catalog).with_context(|| {
                    format!(
                        "could not compare output {} with catalog {}",
                        output.display(),
                        catalog.display()
                    )
                })?);
        if aliases_catalog {
            bail!(
                "refusing to overwrite the input Lightroom catalog with a report: {}",
                output.display()
            );
        }
    }
    Ok(())
}

/// Resolve symlinks in every existing portion of a prospective output path.
/// `canonicalize` cannot resolve a file whose final directories do not exist,
/// so walk upward to the deepest existing ancestor and append the missing tail.
fn resolve_with_existing_ancestor(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut ancestor = absolute.as_path();
    let mut tail = Vec::new();

    loop {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => {
                let mut resolved = fs::canonicalize(ancestor).with_context(|| {
                    format!(
                        "could not resolve existing output path {}",
                        ancestor.display()
                    )
                })?;
                for component in tail.iter().rev() {
                    resolved.push(component);
                }
                return normalize_absolute(&resolved);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let name = ancestor
                    .file_name()
                    .with_context(|| format!("could not resolve output path {}", path.display()))?;
                tail.push(name.to_os_string());
                ancestor = ancestor
                    .parent()
                    .with_context(|| format!("could not resolve output path {}", path.display()))?;
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("could not inspect output path {}", ancestor.display())
                });
            }
        }
    }
}

fn normalize_absolute(path: &Path) -> Result<PathBuf> {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    bail!(
                        "output path escapes its filesystem root: {}",
                        path.display()
                    );
                }
            }
        }
    }
    Ok(normalized)
}
