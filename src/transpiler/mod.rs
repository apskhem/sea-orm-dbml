use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::{path::Path, fs};

use crate::ast::schema::*;
use crate::parser::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
  SeaORMPostgreSQL
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config  {
  in_path: OsString,
  out_path: Option<OsString>,
  target: Target
}

impl Config {
  pub fn new(in_path: impl AsRef<Path>, target: Target) -> Self {
    
    Self {
      in_path: in_path.as_ref().into(),
      out_path: None,
      target
    }
  }

  pub fn set_out_path(mut self, path: impl AsRef<Path>) -> Self {
    self.out_path = Some(path.as_ref().into());

    self
  }

  pub fn transpile(&self) {
    let raw_in = fs::read_to_string(&self.in_path).expect("cannot read the input file");

    let out_ast = parse(&raw_in).expect("cannot parse the input file");

    let sem_ast = out_ast.into_semantic();
    
    let result = transpile(sem_ast, &self.target).expect("cannot transpile the input file");

    let out_path = self.out_path.clone().unwrap_or_else(|| {
      env::var_os("OUT_DIR")
        .ok_or_else(|| {
            Error::new(ErrorKind::Other, "OUT_DIR environment variable is not set")
        })
        .unwrap()
    });

    File::create(out_path)
      .expect("cannot create source module file")
      .write_all(result.as_bytes())
      .expect("error while writing to file");
  }
}

fn transpile(ast: SematicSchemaBlock, target: &Target) -> Result<String, String> {
  match target {
    Target::SeaORMPostgreSQL => transpile_sea_orm_postgresql(ast)
  }
}

fn transpile_sea_orm_postgresql(ast: SematicSchemaBlock) -> Result<String, String> {

  Ok(String::new())
}
