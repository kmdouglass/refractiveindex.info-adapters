use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("refractiveindex.info-database/database")
}

fn ria() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ria"))
}

#[test]
fn test_store_nk_creates_json() {
    let output = env::temp_dir().join("ria_test_store_nk.dat");
    let status = ria()
        .args(["store", "--path", db_path().to_str().unwrap(), "--output", output.to_str().unwrap()])
        .status()
        .expect("failed to run ria");

    assert!(status.success(), "ria store exited with non-zero status");
    let content = fs::read(&output).expect("output file not found");
    assert!(!content.is_empty(), "output file is empty");
    assert_eq!(content[0], b'{', "output does not start with '{{': got {:?}", &content[..content.len().min(20)]);

    fs::remove_file(&output).ok();
}

#[test]
fn test_store_then_validate_roundtrip() {
    let output = env::temp_dir().join("ria_test_roundtrip.dat");
    let store_status = ria()
        .args(["store", "--path", db_path().to_str().unwrap(), "--output", output.to_str().unwrap()])
        .status()
        .expect("failed to run ria store");
    assert!(store_status.success(), "ria store failed");

    let validate_status = ria()
        .args(["validate", "--input", output.to_str().unwrap()])
        .status()
        .expect("failed to run ria validate");
    assert!(validate_status.success(), "ria validate failed");

    fs::remove_file(&output).ok();
}

#[test]
fn test_store_bitcode_then_validate() {
    let output = env::temp_dir().join("ria_test_bitcode.dat");
    let store_status = ria()
        .args(["-f", "bitcode", "store", "--path", db_path().to_str().unwrap(), "--output", output.to_str().unwrap()])
        .status()
        .expect("failed to run ria store");
    assert!(store_status.success(), "ria store (bitcode) failed");

    let validate_status = ria()
        .args(["-f", "bitcode", "validate", "--input", output.to_str().unwrap()])
        .status()
        .expect("failed to run ria validate");
    assert!(validate_status.success(), "ria validate (bitcode) failed");

    fs::remove_file(&output).ok();
}

#[test]
fn test_store_with_include_filter() {
    let include_file = env::temp_dir().join("ria_test_include.txt");
    let output = env::temp_dir().join("ria_test_include_out.dat");

    let mut f = fs::File::create(&include_file).expect("failed to create include file");
    writeln!(f, "main:Ag:Johnson").expect("failed to write include file");

    let status = ria()
        .args([
            "store",
            "--path", db_path().to_str().unwrap(),
            "--output", output.to_str().unwrap(),
            "--include", include_file.to_str().unwrap(),
        ])
        .status()
        .expect("failed to run ria store");
    assert!(status.success(), "ria store with include failed");

    let content = fs::read_to_string(&output).expect("output file not found");
    let json: serde_json::Value = serde_json::from_str(&content).expect("invalid JSON output");
    let inner = json.get("inner").expect("missing 'inner' key");
    let obj = inner.as_object().expect("'inner' is not an object");
    assert_eq!(obj.len(), 1, "expected exactly one entry, got {}", obj.len());
    assert!(obj.contains_key("main:Ag:Johnson"), "key 'main:Ag:Johnson' not found");

    fs::remove_file(&include_file).ok();
    fs::remove_file(&output).ok();
}

#[test]
fn test_store_with_exclude_filter() {
    let exclude_file = env::temp_dir().join("ria_test_exclude.txt");
    let output = env::temp_dir().join("ria_test_exclude_out.dat");

    let mut f = fs::File::create(&exclude_file).expect("failed to create exclude file");
    writeln!(f, "main:Ag:Johnson").expect("failed to write exclude file");

    let status = ria()
        .args([
            "store",
            "--path", db_path().to_str().unwrap(),
            "--output", output.to_str().unwrap(),
            "--exclude", exclude_file.to_str().unwrap(),
        ])
        .status()
        .expect("failed to run ria store");
    assert!(status.success(), "ria store with exclude failed");

    let content = fs::read_to_string(&output).expect("output file not found");
    let json: serde_json::Value = serde_json::from_str(&content).expect("invalid JSON output");
    let inner = json.get("inner").expect("missing 'inner' key");
    let obj = inner.as_object().expect("'inner' is not an object");
    assert!(!obj.contains_key("main:Ag:Johnson"), "excluded key 'main:Ag:Johnson' still present");

    fs::remove_file(&exclude_file).ok();
    fs::remove_file(&output).ok();
}
