use sea_orm_dbml::*;

use std::fs;

fn main() -> Result<(), String> {
  let path = "tests/sample_1.dbml";
  let contents = fs::read_to_string(path).expect("Should have been able to read the file");

  let ast_out = parser::parse(&contents);

  ast_out.unwrap().print();

  Ok(())
}
