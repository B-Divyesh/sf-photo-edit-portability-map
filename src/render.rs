use crate::model::{Location, Report, Support};
use std::fmt::Write;

pub fn render_text(report: &Report) -> String {
    let mut output = String::new();
    let summary = &report.summary;
    writeln!(output, "EDIT PORTABILITY MAP  /  {}", report.target_app).unwrap();
    writeln!(output, "{}", "─".repeat(72)).unwrap();
    writeln!(
        output,
        "Source {:>6}   Target {:>6}   Matched {:>6}   Missing {:>6}",
        summary.source_assets,
        summary.target_assets,
        summary.matched_assets,
        summary.missing_assets
    )
    .unwrap();
    writeln!(
        output,
        "XMP    {:>6}   Catalog records {:>6}   Catalog-only fields {:>3}",
        summary.xmp_sidecars, summary.catalog_records, summary.catalog_only_fields
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "PORTABILITY INVENTORY").unwrap();
    writeln!(
        output,
        "  E embedded   S sidecar   C catalog-only   ! unsupported"
    )
    .unwrap();
    for category in &report.categories {
        let place = match category.location {
            Location::Embedded => "E",
            Location::Sidecar => "S",
            Location::CatalogOnly => "C",
        };
        let support = match category.target_support {
            Support::Unsupported => "! unsupported",
            Support::Partial => "~ partial",
            Support::Unknown => "? verify",
            Support::Supported => "✓ supported",
        };
        writeln!(
            output,
            "  {place}  {:<30} {:>6}   {support}",
            category.label, category.records
        )
        .unwrap();
        writeln!(output, "     {}", category.action).unwrap();
    }
    writeln!(output).unwrap();
    writeln!(output, "MIGRATION CHECKLIST").unwrap();
    for (index, item) in report.checklist.iter().enumerate() {
        writeln!(
            output,
            "  {:>2}. [{}] {}",
            index + 1,
            item.priority.to_ascii_uppercase(),
            item.task
        )
        .unwrap();
        writeln!(output, "      {}", item.why).unwrap();
    }
    if !report.verification_sample.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "VERIFICATION SAMPLE").unwrap();
        for item in &report.verification_sample {
            writeln!(
                output,
                "  [{}] {}",
                item.status.to_ascii_uppercase(),
                item.source
            )
            .unwrap();
            if let Some(target) = &item.target {
                writeln!(output, "      → {target}").unwrap();
            }
            if !item.verify.is_empty() {
                writeln!(output, "      Check: {}", item.verify.join(", ")).unwrap();
            }
        }
    }
    if !report.warnings.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "WARNINGS").unwrap();
        for warning in &report.warnings {
            writeln!(output, "  • {warning}").unwrap();
        }
    }
    writeln!(output).unwrap();
    writeln!(
        output,
        "Read-only report. Keep originals and your catalog until the sample is verified."
    )
    .unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::render_text;
    use crate::model::{Inputs, Report, Summary, TargetApp};

    #[test]
    fn human_report_has_safety_footer() {
        let report = Report {
            schema_version: "1.0".into(),
            generated_at_unix: 0,
            target_app: TargetApp::Generic,
            inputs: Inputs {
                source: "a".into(),
                target: "b".into(),
                catalog: None,
            },
            summary: Summary::default(),
            categories: vec![],
            checklist: vec![],
            verification_sample: vec![],
            warnings: vec![],
        };
        assert!(render_text(&report).contains("Keep originals"));
    }
}
