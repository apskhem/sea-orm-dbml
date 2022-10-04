use sea_orm_dbml::*;

fn main() {
  config("tests/sample_1.dbml", transpiler::Target::SeaORMPostgreSQL)
    .set_out_path("dist/out.rs")
    .transpile();
}
