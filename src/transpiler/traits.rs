use inflector::Inflector;

use crate::ast::*;

pub trait ToProgType {
  fn to_rust_sea_orm_type(&self) -> String;
}

impl ToProgType for table::ColumnType {
  fn to_rust_sea_orm_type(&self) -> String {
    match self {
      Self::Enum(name) => format!("super::{}", name.to_pascal_case()),
      Self::Char => format!("String"),
      Self::VarChar => format!("String"),
      Self::SmallInt => format!("i16"),
      Self::Integer => format!("i32"),
      Self::BigInt => format!("i64"),
      Self::Real => format!("f32"),
      Self::DoublePrecision => format!("f64"),
      Self::Bool => format!("bool"),
      Self::ByteArray => format!("Vec<u8>"),
      Self::Date => format!("Date"),
      Self::Text => format!("String"),
      Self::Time => format!("Time"),
      Self::Timestamp => format!("DateTime"),
      Self::Timestampz => format!("DateTimeWithTimeZone"),
      Self::Uuid => format!("Uuid"),
      Self::Json => format!("Json"),
      Self::Decimal => format!("Decimal"),
      _ => panic!("cannot_format_type_to_seaorm_type")
    }
  }
}