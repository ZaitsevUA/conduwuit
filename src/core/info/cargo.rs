//! Information about the build related to Cargo. This is a frontend interface
//! informed by proc-macros that capture raw information at build time which is
//! further processed at runtime either during static initialization or as
//! necessary.

use std::sync::OnceLock;

use cargo_toml::Manifest;
use conduit_macros::cargo_manifest;

use crate::Result;

// Raw captures of the cargo manifest for each crate. This is provided by a
// proc-macro at build time since the source directory and the cargo toml's may
// not be present during execution.

#[cargo_manifest]
const WORKSPACE_MANIFEST: &'static str = ();
#[cargo_manifest("macros")]
const MACROS_MANIFEST: &'static str = ();
#[cargo_manifest("core")]
const CORE_MANIFEST: &'static str = ();
#[cargo_manifest("database")]
const DATABASE_MANIFEST: &'static str = ();
#[cargo_manifest("service")]
const SERVICE_MANIFEST: &'static str = ();
#[cargo_manifest("admin")]
const ADMIN_MANIFEST: &'static str = ();
#[cargo_manifest("router")]
const ROUTER_MANIFEST: &'static str = ();
#[cargo_manifest("main")]
const MAIN_MANIFEST: &'static str = ();

/// Processed list of features access all project crates. This is generated from
/// the data in the MANIFEST strings and contains all possible project features.
/// For *enabled* features see the info::rustc module instead.
static FEATURES: OnceLock<Vec<String>> = OnceLock::new();

/// List of all possible features for the project. For *enabled* features in
/// this build see the companion function in info::rustc.
pub fn features() -> &'static Vec<String> {
	FEATURES.get_or_init(|| init_features().unwrap_or_else(|e| panic!("Failed initialize features: {e}")))
}

fn init_features() -> Result<Vec<String>> {
	let mut features = Vec::new();
	append_features(&mut features, WORKSPACE_MANIFEST)?;
	append_features(&mut features, MACROS_MANIFEST)?;
	append_features(&mut features, CORE_MANIFEST)?;
	append_features(&mut features, DATABASE_MANIFEST)?;
	append_features(&mut features, SERVICE_MANIFEST)?;
	append_features(&mut features, ADMIN_MANIFEST)?;
	append_features(&mut features, ROUTER_MANIFEST)?;
	append_features(&mut features, MAIN_MANIFEST)?;
	features.sort();
	features.dedup();

	Ok(features)
}

fn append_features(features: &mut Vec<String>, manifest: &str) -> Result<()> {
	let manifest = Manifest::from_str(manifest)?;
	features.extend(manifest.features.keys().cloned());

	Ok(())
}