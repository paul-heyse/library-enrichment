//! Put the compiler's own `lib/` on the binary's rpath.
//!
//! A `rustc_private` binary links `librustc_driver-*.so` dynamically, and nothing in the
//! default search path finds it. The sysroot is asked for rather than written down: hard-coding
//! a host path is exactly the mistake the surrounding tree avoids everywhere else, and it would
//! also silently point at the wrong toolchain the moment the pin moved.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out = std::process::Command::new(&rustc)
        .args(["--print", "sysroot"])
        .output()
        .expect("asking rustc for its sysroot");
    let sysroot = String::from_utf8(out.stdout).expect("sysroot path is utf-8");
    let sysroot = sysroot.trim();
    println!("cargo::rustc-link-arg=-Wl,-rpath,{sysroot}/lib");
    // The probe passes this to the driver it hosts, so the hosted compilation resolves `core`
    // and `std` against the SAME toolchain that provided `rustc_public`.
    println!("cargo::rustc-env=PROBE_SYSROOT={sysroot}");
}
