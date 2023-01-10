#![forbid(unsafe_code)]
#![forbid(clippy::all)]

pub mod generator;
pub mod transpiler;

use std::path::Path;

pub const DEFAULT_SCHEMA: &'static str = "public";
pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn config(in_path: impl AsRef<Path>, target: transpiler::Target) -> transpiler::Config {
  transpiler::Config::new(in_path, target)
}
