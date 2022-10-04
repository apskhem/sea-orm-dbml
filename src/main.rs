use pest::error::Error;
use sea_orm_dbml::{*, parser::Rule};

use std::fs;

fn main() -> Result<(), Error<Rule>> {
  let path = "tests/sample_1.dbml";
  let contents = fs::read_to_string(path).expect("Should have been able to read the file");

  let out_ast = parser::parse(&contents)?;

  let sem_ast = out_ast.transform_sematic();

  sem_ast.print();

  Ok(())
}
