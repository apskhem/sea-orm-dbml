use sea_orm_dbml::*;

fn main() {
  config("tests/dbml/sample_1.dbml", transpiler::Target::SeaORMPostgreSQL)
    .set_out_path("tests/gen/mod.rs")
    .transpile();
}
