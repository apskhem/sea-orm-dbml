#![forbid(unsafe_code)]
#![forbid(clippy::all)]

use std::env;
use std::error::Error;
use std::fs;

pub mod compiler;
pub mod generator;

pub const DEFAULT_SCHEMA: &str = "public";
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn compile(config: compiler::config::Config) -> Result<(), Box<dyn Error>> {
  if let Some(err_msg) = config.validate() {
    return Err(err_msg.into());
  }

  let sem_ast = dbml_rs::parse_file(&config.in_path)?;

  let result = compiler::compile(sem_ast, &config)?;

  fs::write(config.out_path, result.as_bytes())?;

  Ok(())
}
