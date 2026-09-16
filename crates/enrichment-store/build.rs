//! Capture the complete compiled service definition, including transitive local contracts.
//! This is build provenance, not a service data transformation or a hand-maintained file list.
use sha2::{Digest, Sha256};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    bytes
        .iter()
        .flat_map(|byte| {
            [
                char::from(DIGITS[usize::from(byte >> 4)]),
                char::from(DIGITS[usize::from(byte & 15)]),
            ]
        })
        .collect()
}

fn files(
    directory: &Path,
    output: &mut Vec<PathBuf>,
    accepted: fn(&Path) -> bool,
) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", directory.display());
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            if path.file_name().is_none_or(|name| name != "__pycache__") {
                files(&path, output, accepted)?;
            }
        } else if accepted(&path) {
            output.push(path);
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let root = manifest
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root missing")?;
    let mut inputs = [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "pyproject.toml",
        "uv.lock",
    ]
    .map(|name| root.join(name))
    .to_vec();
    println!("cargo:rerun-if-changed={}", root.join("crates").display());
    for entry in fs::read_dir(root.join("crates"))? {
        let package = entry?.path();
        if package.join("Cargo.toml").is_file() {
            inputs.push(package.join("Cargo.toml"));
            if package.join("build.rs").is_file() {
                inputs.push(package.join("build.rs"));
            }
            files(&package.join("src"), &mut inputs, |_| true)?;
        }
    }
    files(&root.join("python"), &mut inputs, |path| {
        path.extension()
            .is_some_and(|ext| ext == "py" || ext == "arrow" || ext == "json")
    })?;
    files(&root.join("config"), &mut inputs, |path| {
        path.extension().is_some_and(|ext| ext == "toml")
    })?;
    inputs.sort();
    inputs.dedup();
    let mut digest = Sha256::new();
    digest.update(b"native-service-definition/1\0");
    let mut build: std::collections::BTreeMap<String, String> = env::vars()
        .filter(|(key, _)| {
            key.starts_with("CARGO_CFG_")
                || key.starts_with("CARGO_FEATURE_")
                || [
                    "TARGET",
                    "PROFILE",
                    "OPT_LEVEL",
                    "DEBUG",
                    "CARGO_ENCODED_RUSTFLAGS",
                ]
                .contains(&key.as_str())
        })
        .collect();
    let compiler = std::process::Command::new(env::var("RUSTC")?)
        .args(["--version", "--verbose"])
        .output()?;
    if !compiler.status.success() {
        return Err("cannot identify selected Rust compiler".into());
    }
    build.insert("rustc".into(), String::from_utf8(compiler.stdout)?);
    let build_bytes = serde_json::to_vec(&build)?;
    digest.update((build_bytes.len() as u64).to_le_bytes());
    digest.update(&build_bytes);
    let mut receipt = Vec::with_capacity(inputs.len());
    for path in inputs {
        println!("cargo:rerun-if-changed={}", path.display());
        let relative = path
            .strip_prefix(root)?
            .to_str()
            .ok_or("non UTF-8 source path")?;
        let bytes = fs::read(&path)?;
        digest.update((relative.len() as u64).to_le_bytes());
        digest.update(relative.as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(&bytes);
        receipt.push(serde_json::json!({"path":relative,"bytes":bytes.len(),"sha256":hex(&Sha256::digest(&bytes))}));
    }
    let digest = hex(&digest.finalize());
    println!("cargo:rustc-env=ENR_NATIVE_SOURCE_DIGEST={digest}");
    let output = PathBuf::from(env::var("OUT_DIR")?).join("native-source-receipt.json");
    let receipt = serde_json::to_vec(
        &serde_json::json!({"format":"native-service-definition/1","digest":digest,"build":build,"inputs":receipt}),
    )?;
    if fs::read(&output).ok().as_deref() != Some(receipt.as_slice()) {
        fs::write(output, receipt)?;
    }
    Ok(())
}
