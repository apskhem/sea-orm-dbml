use inflector::Inflector;

use dbml_rs::ast::*;

pub trait ToProgType {
  fn to_rust_sea_orm_type(&self) -> String;
}

pub trait ToColType {
  fn to_col_type(&self, args: &Vec<table::Value>) -> Option<String>;
}

impl ToProgType for table::ColumnTypeName {
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

impl ToColType for table::ColumnTypeName {
  fn to_col_type(&self, args: &Vec<table::Value>) -> Option<String> {
    let str_arg_vec: Vec<_> = args.iter().map(|arg| arg.to_string()).collect();
    let str_arg = if str_arg_vec.len() == 0 {
      format!("None")
    } else if str_arg_vec.len() == 1 {
      format!("Some({})", str_arg_vec.join(", "))
    } else {
      format!("Some(({}))", str_arg_vec.join(", "))
    };

    match self {
      Self::Char => Some(format!("Char({})", str_arg)),
      Self::VarChar => Some(format!("String({})", str_arg)),
      Self::SmallInt => Some(format!("SmallInteger")),
      Self::Integer => Some(format!("Integer")),
      Self::BigInt => Some(format!("BigInteger")),
      Self::Real => Some(format!("Float")),
      Self::DoublePrecision => Some(format!("Double")),
      Self::Bool => Some(format!("Boolean")),
      Self::ByteArray => Some(format!("Binary")),
      Self::Date => Some(format!("Date")),
      Self::Text => Some(format!("Text")),
      Self::Time => Some(format!("Time")),
      Self::Timestamp => Some(format!("DateTime")),
      Self::Timestampz => Some(format!("TimestampWithTimeZone")),
      Self::Uuid => Some(format!("Uuid")),
      Self::Json => Some(format!("Json")),
      Self::Decimal => Some(format!("Decimal({})", str_arg)),
      _ => None
    }
  }
}