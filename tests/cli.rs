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
fn mixed_catalog_and_xmp_coverage_is_a_catalog_only_blocker() {
    let (temp, source, target) = fixture();
    fs::write(source.join("second.CR3"), b"not opened").unwrap();
    fs::write(target.join("second.jpg"), b"not opened").unwrap();
    let catalog = temp.path().join("library.lrcat");
    let connection = rusqlite::Connection::open(&catalog).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE Adobe_images (id INTEGER PRIMARY KEY, rating INTEGER);\
             INSERT INTO Adobe_images (rating) VALUES (5), (4);",
        )
        .unwrap();
    drop(connection);

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--catalog",
            catalog.to_str().unwrap(),
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--target-app",
            "immich",
            "--json",
            "--fail-on-blockers",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(3));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["catalog_only_fields"], 1);
    let rating = report["categories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["field"] == "rating")
        .unwrap();
    assert_eq!(rating["location"], "catalog_only");
    assert_eq!(rating["records"], 1);
    assert!(
        rating["detail"]
            .as_str()
            .unwrap()
            .contains("2 populated catalog record(s), 1 XMP sidecar(s)")
    );
    assert!(report["checklist"].as_array().unwrap().iter().any(|item| {
        item["priority"] == "blocker" && item["task"].as_str().unwrap().contains("Star rating")
    }));
}

#[test]
fn orphan_xmp_cannot_hide_catalog_only_metadata() {
    let temp = tempdir().unwrap();
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&target).unwrap();
    for name in ["A", "B"] {
        fs::write(source.join(format!("{name}.CR3")), b"not opened").unwrap();
        fs::write(target.join(format!("{name}.jpg")), b"not opened").unwrap();
    }
    fs::write(
        source.join("orphan.xmp"),
        r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="5" /></rdf:RDF></x:xmpmeta>"#,
    )
    .unwrap();
    let catalog = temp.path().join("library.lrcat");
    let connection = rusqlite::Connection::open(&catalog).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE Adobe_images (id INTEGER PRIMARY KEY, rating INTEGER);\
             INSERT INTO Adobe_images (rating) VALUES (5);",
        )
        .unwrap();
    drop(connection);

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--catalog",
            catalog.to_str().unwrap(),
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
            "--fail-on-blockers",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(3));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["xmp_sidecars"], 0);
    assert_eq!(report["summary"]["catalog_only_fields"], 1);
    let rating = report["categories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["field"] == "rating")
        .unwrap();
    assert_eq!(rating["location"], "catalog_only");
    assert_eq!(rating["records"], 1);
    assert!(report["warnings"].as_array().unwrap().iter().any(|item| {
        item.as_str()
            .unwrap()
            .contains("Ignored orphan XMP orphan.xmp")
    }));
}

#[test]
fn xmp_association_handles_case_pairs_and_relocated_orphans() {
    let temp = tempdir().unwrap();
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    fs::create_dir_all(source.join("Album")).unwrap();
    fs::create_dir_all(source.join("Relocated")).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(source.join("Album/PHOTO.CR3"), b"not opened").unwrap();
    fs::write(source.join("pair.CR3"), b"not opened").unwrap();
    fs::write(source.join("pair.JPG"), b"paired image is a separate asset").unwrap();
    let rating_xmp = r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="5" /></rdf:RDF></x:xmpmeta>"#;
    fs::write(source.join("Album/photo.XMP"), rating_xmp).unwrap();
    fs::write(source.join("pair.xmp"), rating_xmp).unwrap();
    fs::write(source.join("Relocated/PHOTO.xmp"), rating_xmp).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["source_assets"], 3);
    assert_eq!(report["summary"]["xmp_sidecars"], 2);
    let rating = report["categories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["field"] == "rating")
        .unwrap();
    assert_eq!(rating["records"], 2);
    assert!(report["warnings"].as_array().unwrap().iter().any(|item| {
        item.as_str()
            .unwrap()
            .contains("Ignored orphan XMP Relocated/PHOTO.xmp")
    }));
}

#[test]
fn mixed_valid_and_orphan_xmp_only_offsets_valid_coverage() {
    let temp = tempdir().unwrap();
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(source.join("valid.CR3"), b"not opened").unwrap();
    fs::write(target.join("valid.jpg"), b"not opened").unwrap();
    let rating_xmp = r#"<x:xmpmeta xmlns:x="x" xmlns:xmp="x"><rdf:RDF xmlns:rdf="r"><rdf:Description xmp:Rating="5" /></rdf:RDF></x:xmpmeta>"#;
    fs::write(source.join("valid.xmp"), rating_xmp).unwrap();
    fs::write(source.join("orphan.xmp"), rating_xmp).unwrap();
    let catalog = temp.path().join("library.lrcat");
    let connection = rusqlite::Connection::open(&catalog).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE Adobe_images (id INTEGER PRIMARY KEY, rating INTEGER);\
             INSERT INTO Adobe_images (rating) VALUES (5), (4);",
        )
        .unwrap();
    drop(connection);

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--catalog",
            catalog.to_str().unwrap(),
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
            "--fail-on-blockers",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["xmp_sidecars"], 1);
    let rating = report["categories"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["field"] == "rating")
        .unwrap();
    assert_eq!(rating["location"], "catalog_only");
    assert_eq!(rating["records"], 1);
}

#[test]
fn unique_file_name_matches_a_safely_flattened_target() {
    let temp = tempdir().unwrap();
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    fs::create_dir_all(source.join("2024/Trip")).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(source.join("2024/Trip/DSC_0042.NEF"), b"not opened").unwrap();
    fs::write(target.join("DSC_0042.jpg"), b"not opened").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["matched_assets"], 1);
    assert_eq!(report["summary"]["missing_assets"], 0);
    assert_eq!(report["verification_sample"][0]["target"], "DSC_0042.jpg");
}

#[test]
fn duplicate_file_names_do_not_use_the_flattened_fallback() {
    let temp = tempdir().unwrap();
    let source = temp.path().join("source");
    let target = temp.path().join("target");
    fs::create_dir_all(source.join("one")).unwrap();
    fs::create_dir_all(source.join("two")).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(source.join("one/DSC_0042.NEF"), b"not opened").unwrap();
    fs::write(source.join("two/DSC_0042.NEF"), b"not opened").unwrap();
    fs::write(target.join("DSC_0042.jpg"), b"not opened").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["matched_assets"], 0);
    assert_eq!(report["summary"]["missing_assets"], 2);
}

#[test]
fn paired_raw_and_jpeg_targets_are_assigned_one_to_one() {
    for (target_names, matched, missing) in [
        (&["photo.JPG"][..], 1, 1),
        (&["photo.CR3"][..], 1, 1),
        (&["photo.webp"][..], 0, 2),
        (&["photo.CR3", "photo.JPG"][..], 2, 0),
    ] {
        let temp = tempdir().unwrap();
        let source = temp.path().join("source");
        let target = temp.path().join("target");
        fs::create_dir_all(&source).unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::write(source.join("photo.CR3"), b"raw is a separate asset").unwrap();
        fs::write(source.join("photo.JPG"), b"jpeg is a separate asset").unwrap();
        for name in target_names {
            fs::write(target.join(name), b"not opened").unwrap();
        }

        let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
            .args([
                "scan",
                "--source",
                source.to_str().unwrap(),
                "--target",
                target.to_str().unwrap(),
                "--json",
                "--fail-on-blockers",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(if missing == 0 { 0 } else { 3 }));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["summary"]["matched_assets"], matched);
        assert_eq!(report["summary"]["missing_assets"], missing);
        let assigned_targets: Vec<&str> = report["verification_sample"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["target"].as_str())
            .collect();
        let unique_targets: std::collections::BTreeSet<&str> =
            assigned_targets.iter().copied().collect();
        assert_eq!(assigned_targets.len(), unique_targets.len());
        if missing > 0 && !target_names.is_empty() {
            assert!(report["warnings"].as_array().unwrap().iter().any(|item| {
                let warning = item.as_str().unwrap();
                warning.contains("targets are never reused")
                    || warning.contains("never guessed or reused")
            }));
        }
    }
}

#[test]
fn truncated_xmp_is_counted_but_warned_and_contributes_no_fields() {
    let (_temp, source, target) = fixture();
    fs::write(source.join("frame.xmp"), "<x:xmpmeta><unclosed>").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_edit-portability-map"))
        .args([
            "scan",
            "--source",
            source.to_str().unwrap(),
            "--target",
            target.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["xmp_sidecars"], 1);
    assert!(
        report["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|warning| {
                warning
                    .as_str()
                    .unwrap()
                    .contains("Skipped malformed XMP frame.xmp")
                    && warning.as_str().unwrap().contains("unexpected end of file")
            })
    );
    assert!(
        report["categories"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["field"] != "rating")
    );
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
fn sample_size_above_hard_maximum_is_rejected_before_license_gate() {
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
            "101",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr.contains("sample size cannot exceed 100"));
    assert!(!stderr.contains("Pro license"));
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
