use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use edit_portability_map::{ScanOptions, TargetApp, license, render_text, scan};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "edit-portability-map",
    version,
    about = "See which Lightroom and XMP state will survive a photo-library move",
    long_about = "Read-only migration pre-flight for Lightroom catalogs, XMP sidecars, and a target folder. It inventories metadata ownership without decoding image pixels or modifying source files."
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
    /// Activate or inspect the optional one-time Pro license
    License {
        #[command(subcommand)]
        command: LicenseCommand,
    },
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
            if sample_size > 10 {
                license::require_pro()?;
            }
            for output in [&report, &json_report].into_iter().flatten() {
                ensure_output_is_safe(output, &source, &target)?;
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
    }
}

fn ensure_output_is_safe(output: &PathBuf, source: &PathBuf, target: &PathBuf) -> Result<()> {
    let absolute_output = absolute_without_existing(output)?;
    for input in [source, target] {
        let canonical = fs::canonicalize(input).unwrap_or_else(|_| input.clone());
        if absolute_output.starts_with(&canonical) {
            bail!(
                "refusing to write report inside the scanned {} folder: {}",
                if input == source { "source" } else { "target" },
                output.display()
            );
        }
    }
    Ok(())
}

fn absolute_without_existing(path: &PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.clone())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}
