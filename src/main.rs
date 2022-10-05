use std::io::Result;

use dbml_entity::*;

fn main() -> Result<()> {
  config("tests/dbml/sample_1.dbml", transpiler::Target::SeaORMPostgreSQL)
    .set_out_path("tests/gen/mod.rs")
    .transpile()
}
