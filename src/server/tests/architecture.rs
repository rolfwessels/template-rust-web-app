use serde_json::Value;
use std::{path::Path, process::Command};

// Keep this list small and limited to framework-independent libraries.
const CORE_DEPENDENCIES: &[&str] = &["futures-core"];

#[test]
fn workspace_preserves_the_core_boundary() {
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version=1",
            "--no-deps",
            "--locked",
            "--offline",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata: Value = serde_json::from_slice(&output.stdout).expect("parse cargo metadata");
    let root = Path::new(metadata["workspace_root"].as_str().expect("workspace root"));
    let packages = metadata["packages"].as_array().expect("workspace packages");
    let package_at = |manifest: &Path| {
        packages
            .iter()
            .find(|package| package["manifest_path"].as_str().map(Path::new) == Some(manifest))
            .unwrap_or_else(|| panic!("missing workspace package at {}", manifest.display()))
    };

    let core_dir = root.join("src/core");
    let core = package_at(&core_dir.join("Cargo.toml"));
    let unexpected: Vec<_> = core["dependencies"]
        .as_array()
        .expect("core dependencies")
        .iter()
        .filter(|dependency| dependency["kind"] != "dev")
        .map(|dependency| dependency["name"].as_str().expect("dependency name"))
        .filter(|name| !CORE_DEPENDENCIES.contains(name))
        .collect();
    assert!(
        unexpected.is_empty(),
        "the application core has unexpected production dependencies: {}. Keep infrastructure in the server crate.",
        unexpected.join(", ")
    );

    let server = package_at(&root.join("src/server/Cargo.toml"));
    assert!(
        server["dependencies"]
            .as_array()
            .expect("server dependencies")
            .iter()
            .any(|dependency| {
                dependency["kind"].is_null()
                    && dependency["name"] == core["name"]
                    && dependency["path"].as_str().map(Path::new) == Some(core_dir.as_path())
            }),
        "the server must have a production dependency on the local application core"
    );
}
