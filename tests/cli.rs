use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let temp = tempdir().unwrap();
    let source = temp.path().join("Originals");
    let target = temp.path().join("Export");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(source.join("frame.jpg"), b"pixel bytes are never parsed").unwrap();
    fs::write(target.join("frame.webp"), b"pixel bytes are never parsed").unwrap();
    fs::write(
        source.join("frame.xmp"),
        r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="4" /></rdf:RDF></x:xmpmeta>"#,
    )
    .unwrap();
    (temp, source, target)
}

#[test]
fn documented_json_scan_is_scriptable() {
    let (_temp, source, target) = fixture();
    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--target-app",
            "darktable",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema_version"], "1.0");
    assert_eq!(report["summary"]["matched_assets"], 1);
}

#[test]
fn blocker_mode_uses_exit_code_three() {
    let (_temp, source, target) = fixture();
    fs::remove_file(target.join("frame.webp")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--fail-on-blockers",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn pro_sized_sample_requires_a_license() {
    let (_temp, source, target) = fixture();
    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .env_remove("EDIT_PORTABILITY_MAP_LICENSE")
        .env("XDG_CONFIG_HOME", _temp.path().join("empty-config"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--sample-size",
            "11",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Pro license is required"));
}

#[test]
fn report_cannot_alias_and_overwrite_the_input_catalog() {
    let (temp, source, target) = fixture();
    let catalog = temp.path().join("library.lrcat");
    let connection = rusqlite::Connection::open(&catalog).unwrap();
    connection
        .execute("CREATE TABLE Adobe_images (id INTEGER PRIMARY KEY)", [])
        .unwrap();
    connection
        .execute("INSERT INTO Adobe_images DEFAULT VALUES", [])
        .unwrap();
    drop(connection);
    let before = fs::read(&catalog).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--catalog",
            catalog.to_str().unwrap(),
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--report",
            catalog.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("refusing to overwrite the input Lightroom catalog")
    );
    assert_eq!(fs::read(&catalog).unwrap(), before);
    let connection =
        rusqlite::Connection::open_with_flags(&catalog, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .unwrap();
    let rows: i64 = connection
        .query_row("SELECT COUNT(*) FROM Adobe_images", [], |row| row.get(0))
        .unwrap();
    assert_eq!(rows, 1);
}

#[cfg(unix)]
#[test]
fn report_cannot_enter_source_through_a_symlinked_source_path() {
    use std::os::unix::fs::symlink;

    let (temp, source, target) = fixture();
    let linked_source = temp.path().join("linked-source");
    symlink(&source, &linked_source).unwrap();
    let report = linked_source.join("nested").join("report.txt");

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            linked_source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--report",
            report.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("refusing to write report inside the scanned source folder")
    );
    assert!(!source.join("nested").join("report.txt").exists());
}
