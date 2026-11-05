//! End-to-end checks for `pantheon-journal validate`: every issue in one run,
//! `--json`, and the exit code.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-journal");

const GOOD: &str = "---\ntitle: \"Example\"\ndate: 2026-07-22\ntags:\n  - journal\n---\n\n# Example - July 22, 2026\n\n## Session Overview\n\nText.\n\n## Compliance\n\nText.\n\n## Tags\n\n`journal`\n";

fn write(root: &Path, rel: &str, content: &str) -> PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, content).unwrap();
    path
}

fn validate(path: &Path, extra: &[&str]) -> Output {
    Command::new(BIN)
        .arg("validate")
        .arg(path)
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn a_valid_entry_exits_zero() {
    let tmp = tempfile::tempdir().unwrap();
    let path = write(tmp.path(), "2026/07/2026-07-22-example.md", GOOD);
    let out = validate(&path, &[]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).starts_with("OK:"));
}

#[test]
fn an_invalid_entry_lists_every_problem_and_exits_one() {
    let tmp = tempfile::tempdir().unwrap();
    let body = GOOD
        .replace("title: \"Example\"\n", "")
        .replace("## Compliance\n\nText.\n\n", "");
    let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
    let out = validate(&path, &[]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("title"), "{stderr}");
    assert!(stderr.contains("## Compliance"), "{stderr}");
}

#[test]
fn json_reports_every_issue_with_a_location() {
    let tmp = tempfile::tempdir().unwrap();
    let body = GOOD
        .replace("title: \"Example\"\n", "")
        .replace("## Compliance\n\nText.\n\n", "");
    let path = write(tmp.path(), "2026/07/2026-07-22-example.md", &body);
    let out = validate(&path, &["--json"]);
    assert_eq!(out.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["result"], "invalid");
    let issues = report["issues"].as_array().unwrap();
    assert!(issues.len() >= 2, "{report}");
    assert!(issues
        .iter()
        .all(|i| i["location"].is_string() && i["message"].is_string()));
    assert!(issues
        .iter()
        .any(|i| i["location"].as_str().unwrap().starts_with("frontmatter")));
}

#[test]
fn json_for_a_valid_entry_has_no_issues_and_exits_zero() {
    let tmp = tempfile::tempdir().unwrap();
    let path = write(tmp.path(), "2026/07/2026-07-22-example.md", GOOD);
    let out = validate(&path, &["--json"]);
    assert!(out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["result"], "valid");
    assert_eq!(report["issues"].as_array().unwrap().len(), 0);
}

#[test]
fn json_for_a_file_outside_a_year_tree_is_skipped() {
    let tmp = tempfile::tempdir().unwrap();
    let path = write(tmp.path(), "notes/2026-07-22-example.md", GOOD);
    let out = validate(&path, &["--json"]);
    assert!(out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["result"], "skipped");
}
