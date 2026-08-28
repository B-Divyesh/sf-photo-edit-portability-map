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
