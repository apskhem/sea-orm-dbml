use std::{
  fs,
  io::Result,
  path::{Path, PathBuf},
  ffi::OsString,
};

use sea_orm_dbml::{compiler::config::Config, *};

const DBML_DIR: &str = "tests/dbml";
const OUT_DIR: &str = "tests/out";

fn read_dbml_dir<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>> {
  let mut out = vec![];
  let entries = fs::read_dir(dir_path)?;

  for entry in entries {
    let file_path = entry?.path();

    if file_path.is_file() {
      out.push(file_path);
    }
  }

  Ok(out)
}

fn create_out_dir() -> Result<()> {
  if !fs::metadata(OUT_DIR).is_ok() {
    fs::create_dir(OUT_DIR)?;
  }

  Ok(())
}

#[test]
fn gen_all() -> Result<()> {
  create_out_dir()?;

  let testing_dbml_files = read_dbml_dir(DBML_DIR)?;

  for path in testing_dbml_files {
    let mut out_file_path = path.clone();
    out_file_path.set_extension("rs");
    let out_file_name = out_file_path.file_name().unwrap().to_str().unwrap();

    let res = compile(Config {
      in_path: OsString::from(&path),
      out_path: OsString::from(format!("{}/{}", OUT_DIR, out_file_name)),
      target: compiler::config::Target::Postgres,
      ..Default::default()
    });
    
    res.unwrap_or_else(|err| panic!("{:?} {}", path, err));
  }

  Ok(())
}