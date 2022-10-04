#![forbid(unsafe_code)]
#![forbid(clippy::all)]

#[macro_use] extern crate pest_derive;

pub mod ast;
pub mod error;
pub mod parser;
pub mod generator;
pub mod transpiler;

use std::path::Path;

pub fn config(in_path: impl AsRef<Path>, target: transpiler::Target) -> transpiler::Config {
  transpiler::Config::new(in_path, target)
}
