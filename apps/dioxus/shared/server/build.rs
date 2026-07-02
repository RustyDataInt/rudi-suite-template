//! RuDI shared server `build.rs` converts a developer's 
//! app configuration files into all required server code,
//! including imports, structures, and enums.

// RuDI developer note: this script:
//     `rudi-suite-template/apps/dioxus/shared/server/build.rs`
// rarely changes even if updates are made to the shared builder:
//     `rudi-apps-framework/rudi_apps/src/server/mod.rs::build()`
// so `println!("cargo:rerun-if-changed=build.rs");` doesn't do much.
// When working on changes to the shared `build()` function
// it is best to touch this file when compiling:
//     `touch build.rs && dx build`
// Changes to the user's config files are separately tracked to 
// trigger builds, these comments only apply when updating `build()`.

/// Main server build function. All actions are inherited 
/// from the rudi-apps-framework.
fn main() {
    // set by cargo when compiling a tool suite
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    rudi_apps::server::build(&cargo_manifest_dir, &out_dir);
}
