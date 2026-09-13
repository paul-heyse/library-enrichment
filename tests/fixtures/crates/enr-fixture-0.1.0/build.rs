//! Build script with a detectable side effect (tests/ACCEPTANCE_PLAN.md).
//!
//! Gate R02 asserts that indexing hosted rustdoc JSON never invokes Cargo. If this script ever
//! runs during a test, it writes a marker file into `ENR_FIXTURE_MARKER_DIR`; the test asserts
//! that marker is absent. Absent the variable it writes only to `OUT_DIR`, as any build script
//! may.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    fs::write(out_dir.join("built.txt"), &version).expect("write OUT_DIR marker");

    if let Ok(dir) = env::var("ENR_FIXTURE_MARKER_DIR") {
        let dir = PathBuf::from(dir);
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(dir.join(format!("enr-fixture-{version}.built")), &version);
    }
    println!("cargo:rerun-if-env-changed=ENR_FIXTURE_MARKER_DIR");
}
