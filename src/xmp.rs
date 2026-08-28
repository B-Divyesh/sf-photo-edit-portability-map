use anyhow::{Context, Result};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct XmpInventory {
    pub sidecars: usize,
    pub counts: BTreeMap<String, usize>,
    pub malformed: Vec<String>,
}

pub fn inspect(paths: &[std::path::PathBuf], root: &Path) -> Result<XmpInventory> {
    let mut result = XmpInventory::default();
    for path in paths {
        result.sidecars += 1;
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                let shown = path.strip_prefix(root).unwrap_or(path);
                result
                    .malformed
                    .push(format!("{}: unreadable text ({error})", shown.display()));
                continue;
            }
        };
        match fields_in_document(&text) {
            Ok(fields) => {
                for field in fields {
                    *result.counts.entry(field).or_default() += 1;
                }
            }
            Err(error) => {
                let shown = path.strip_prefix(root).unwrap_or(path);
                result
                    .malformed
                    .push(format!("{}: {error}", shown.display()));
            }
        }
    }
    Ok(result)
}

fn fields_in_document(text: &str) -> Result<BTreeSet<String>> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().trim_text(true);
    let mut fields = BTreeSet::new();
    let mut open_elements = Vec::new();
    let mut saw_root = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) => {
                classify_name(event.name().as_ref(), &mut fields);
                for attribute in event.attributes().with_checks(false) {
                    let attribute = attribute.context("invalid XMP attribute")?;
                    classify_name(attribute.key.as_ref(), &mut fields);
                }
                saw_root = true;
                open_elements.push(event.name().as_ref().to_vec());
            }
            Ok(Event::Empty(event)) => {
                classify_name(event.name().as_ref(), &mut fields);
                for attribute in event.attributes().with_checks(false) {
                    let attribute = attribute.context("invalid XMP attribute")?;
                    classify_name(attribute.key.as_ref(), &mut fields);
                }
                saw_root = true;
            }
            Ok(Event::End(event)) => {
                let expected = open_elements
                    .pop()
                    .context("malformed XML (unexpected closing element)")?;
                if expected != event.name().as_ref() {
                    anyhow::bail!(
                        "malformed XML (expected closing element {}, found {})",
                        String::from_utf8_lossy(&expected),
                        String::from_utf8_lossy(event.name().as_ref())
                    );
                }
            }
            Ok(Event::Eof) if !open_elements.is_empty() => {
                let element = open_elements.last().expect("checked non-empty");
                anyhow::bail!(
                    "malformed XML (unexpected end of file with {} still open)",
                    String::from_utf8_lossy(element)
                );
            }
            Ok(Event::Eof) if !saw_root => anyhow::bail!("malformed XML (no root element)"),
            Ok(Event::Eof) => break,
            Err(error) => anyhow::bail!("malformed XML ({error})"),
            _ => {}
        }
    }
    Ok(fields)
}

fn classify_name(name: &[u8], fields: &mut BTreeSet<String>) {
    let name = String::from_utf8_lossy(name).to_ascii_lowercase();
    let local = name.rsplit(':').next().unwrap_or(&name);
    let field = if matches!(local, "datetimeoriginal" | "datecreated" | "createdate") {
        Some("capture_date")
    } else if local == "rating" {
        Some("rating")
    } else if local == "label" {
        Some("color_label")
    } else if matches!(local, "subject" | "hierarchicalsubject") {
        Some("keywords")
    } else if local == "title" {
        Some("title")
    } else if local == "caption" || (local == "description" && !name.starts_with("rdf:")) {
        Some("caption")
    } else if local.contains("gps") || matches!(local, "location" | "city" | "country") {
        Some("location")
    } else if matches!(local, "rights" | "copyright") {
        Some("copyright")
    } else if local == "orientation" {
        Some("orientation")
    } else if name.starts_with("crs:") || local.contains("develop") {
        Some("develop_recipe")
    } else if matches!(local, "pick" | "rejected") {
        Some("pick_flags")
    } else if local.contains("person") || local.contains("face") {
        Some("people")
    } else {
        None
    };
    if let Some(field) = field {
        fields.insert(field.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::fields_in_document;

    #[test]
    fn recognizes_namespaced_elements_and_attributes() {
        let xmp = r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x" xmlns:dc="d"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="4"><dc:subject><rdf:Bag /></dc:subject></rdf:Description></rdf:RDF></x:xmpmeta>"#;
        let fields = fields_in_document(xmp).unwrap();
        assert!(fields.contains("rating"));
        assert!(fields.contains("keywords"));
        assert!(!fields.contains("caption"));
    }

    #[test]
    fn rejects_truncated_elements_at_end_of_file() {
        let error = fields_in_document("<x:xmpmeta><unclosed>").unwrap_err();
        assert!(error.to_string().contains("unexpected end of file"));
    }
}
