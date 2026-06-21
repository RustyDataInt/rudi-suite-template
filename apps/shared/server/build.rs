//! RuDI shared server `build.rs` converts a developer's 
//! app configuration files into all required server code,
//! including imports, structures, and enums.

/// Main server build function. All actions are inherited 
/// from the rudi-apps-framework.
fn main() {
    // set by cargo when compiling a tool suite
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    rudi::server::build(&cargo_manifest_dir, &out_dir);
}
