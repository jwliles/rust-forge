fn main() {
    // Tell Cargo this script should run again if the man page changes
    println!("cargo:rerun-if-changed=forge.1");

    // Supply the complete path to the man page for installation hooks
    println!(
        "cargo:rustc-env=FORGE_MAN_PAGE={}",
        std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("forge.1")
            .display()
    );
}