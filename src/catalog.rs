use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct CatalogInventory {
    pub records: usize,
    pub tables: usize,
    pub counts: BTreeMap<String, usize>,
    pub recognized: bool,
}

pub fn inspect(path: &Path) -> Result<CatalogInventory> {
    let connection = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .with_context(|| format!("could not open catalog {} read-only", path.display()))?;
    connection.execute_batch("PRAGMA query_only=ON;")?;

    let tables = table_names(&connection)?;
    let mut result = CatalogInventory {
        tables: tables.len(),
        recognized: tables.iter().any(|name| {
            matches!(
                name.as_str(),
                "Adobe_images" | "AgLibraryFile" | "AgLibraryFolder"
            )
        }),
        ..CatalogInventory::default()
    };

    for table in &tables {
        let row_count = count_rows(&connection, table)?;
        if matches!(table.as_str(), "Adobe_images" | "AgLibraryFile") {
            result.records = result.records.max(row_count);
        }
        if let Some(field) = classify_identifier(table) {
            update_max(&mut result.counts, field, row_count);
        }
        for column in columns(&connection, table)? {
            if let Some(field) = classify_identifier(&format!("{table}_{column}")) {
                let populated = count_populated(&connection, table, &column)?;
                update_max(&mut result.counts, field, populated);
            }
        }
    }
    Ok(result)
}

fn update_max(counts: &mut BTreeMap<String, usize>, field: &str, count: usize) {
    let value = counts.entry(field.to_owned()).or_default();
    *value = (*value).max(count);
}

fn table_names(connection: &Connection) -> Result<Vec<String>> {
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )?;
    let names = statement
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;
    Ok(names)
}

fn columns(connection: &Connection, table: &str) -> Result<Vec<String>> {
    let sql = format!("PRAGMA table_info({})", quote_identifier(table));
    let mut statement = connection.prepare(&sql)?;
    Ok(statement
        .query_map([], |row| row.get(1))?
        .collect::<rusqlite::Result<Vec<String>>>()?)
}

fn count_rows(connection: &Connection, table: &str) -> Result<usize> {
    let sql = format!("SELECT COUNT(*) FROM {}", quote_identifier(table));
    connection
        .query_row(&sql, [], |row| row.get(0))
        .with_context(|| format!("could not inventory catalog table {table}"))
}

fn count_populated(connection: &Connection, table: &str, column: &str) -> Result<usize> {
    let column = quote_identifier(column);
    let sql = format!(
        "SELECT COUNT(*) FROM {} WHERE {column} IS NOT NULL AND TRIM(CAST({column} AS TEXT)) NOT IN ('', '0')",
        quote_identifier(table)
    );
    connection
        .query_row(&sql, [], |row| row.get(0))
        .with_context(|| format!("could not inspect {table}.{column}"))
}

fn quote_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn classify_identifier(value: &str) -> Option<&'static str> {
    let value = value.to_ascii_lowercase();
    if value.contains("develophistor") {
        Some("develop_history")
    } else if value.contains("develop") || value.contains("crs") {
        Some("develop_recipe")
    } else if value.contains("virtualcop") || value.contains("masterimage") {
        Some("virtual_copies")
    } else if value.contains("collection") {
        Some("collections")
    } else if value.contains("keyword") || value.contains("tag") {
        Some("keywords")
    } else if value.contains("stack") {
        Some("stacks")
    } else if value.contains("rating") {
        Some("rating")
    } else if value.contains("colorlabel") || value.ends_with("_label") {
        Some("color_label")
    } else if value.contains("pick") || value.contains("reject") {
        Some("pick_flags")
    } else if value.contains("capturetime") || value.contains("datetimeoriginal") {
        Some("capture_date")
    } else if value.contains("caption") || value.contains("description") {
        Some("caption")
    } else if value.contains("title") {
        Some("title")
    } else if value.contains("copyright") || value.contains("rights") {
        Some("copyright")
    } else if value.contains("gps") || value.contains("location") {
        Some("location")
    } else if value.contains("face") || value.contains("person") {
        Some("people")
    } else if value.contains("orientation") {
        Some("orientation")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_identifier, quote_identifier};

    #[test]
    fn maps_lightroom_schema_names() {
        assert_eq!(classify_identifier("Adobe_images_rating"), Some("rating"));
        assert_eq!(
            classify_identifier("Adobe_libraryImageDevelopHistoryStep"),
            Some("develop_history")
        );
    }

    #[test]
    fn safely_quotes_identifiers() {
        assert_eq!(quote_identifier("odd\"name"), "\"odd\"\"name\"");
    }
}
