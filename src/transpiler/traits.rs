use crate::ast::*;

pub trait ToProgType {
  fn to_rust_sea_orm_type(&self) -> String;
}

impl ToProgType for table::ColumnType {
  fn to_rust_sea_orm_type(&self) -> String {
    String::from("u32")
  }
}