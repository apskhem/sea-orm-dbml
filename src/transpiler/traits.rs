use inflector::Inflector;

use dbml_rs::ast::*;

pub trait ToRustType {
  fn to_rust_type(&self) -> String;
}

pub trait ToColType {
  fn to_col_type(&self) -> Option<String>;
}

impl ToRustType for table::ColumnType {
  fn to_rust_type(&self) -> String {
    let str_type = match &self.type_name {
      table::ColumnTypeName::Enum(name) => format!("super::{}", name.to_pascal_case()),
      table::ColumnTypeName::Char => format!("String"),
      table::ColumnTypeName::VarChar => format!("String"),
      table::ColumnTypeName::SmallInt => format!("i16"),
      table::ColumnTypeName::Integer => format!("i32"),
      table::ColumnTypeName::BigInt => format!("i64"),
      table::ColumnTypeName::Real => format!("f32"),
      table::ColumnTypeName::DoublePrecision => format!("f64"),
      table::ColumnTypeName::Bool => format!("bool"),
      table::ColumnTypeName::ByteArray => format!("Vec<u8>"),
      table::ColumnTypeName::Date => format!("Date"),
      table::ColumnTypeName::Text => format!("String"),
      table::ColumnTypeName::Time => format!("Time"),
      table::ColumnTypeName::Timestamp => format!("DateTime"),
      table::ColumnTypeName::Timestamptz => format!("DateTimeWithTimeZone"),
      table::ColumnTypeName::Uuid => format!("Uuid"),
      table::ColumnTypeName::Json => format!("Json"),
      table::ColumnTypeName::Decimal => format!("Decimal"),
      _ => panic!("cannot_format_type_to_seaorm_type")
    };

    self.arrays.iter().fold(str_type, |acc, arr| {
      format!("Vec<{}>", acc)
    })
  }
}

impl ToColType for table::ColumnType {
  fn to_col_type(&self) -> Option<String> {
    let str_arg_vec: Vec<_> = self.args.iter().map(|arg| arg.to_string()).collect();

    let str_arg = match str_arg_vec.len() {
      0 => format!("None"),
      1 => format!("Some({})", str_arg_vec.join(", ")),
      _ => format!("Some(({}))", str_arg_vec.join(", "))
    };

    let str_type = match self.type_name {
      table::ColumnTypeName::Char => Some(format!("Char({})", str_arg)),
      table::ColumnTypeName::VarChar => Some(format!("String({})", str_arg)),
      table::ColumnTypeName::SmallInt => Some(format!("SmallInteger")),
      table::ColumnTypeName::Integer => Some(format!("Integer")),
      table::ColumnTypeName::BigInt => Some(format!("BigInteger")),
      table::ColumnTypeName::Real => Some(format!("Float")),
      table::ColumnTypeName::DoublePrecision => Some(format!("Double")),
      table::ColumnTypeName::Bool => Some(format!("Boolean")),
      table::ColumnTypeName::ByteArray => Some(format!("Binary")),
      table::ColumnTypeName::Date => Some(format!("Date")),
      table::ColumnTypeName::Text => Some(format!("Text")),
      table::ColumnTypeName::Time => Some(format!("Time")),
      table::ColumnTypeName::Timestamp => Some(format!("DateTime")),
      table::ColumnTypeName::Timestamptz => Some(format!("TimestampWithTimeZone")),
      table::ColumnTypeName::Uuid => Some(format!("Uuid")),
      table::ColumnTypeName::Json => Some(format!("Json")),
      table::ColumnTypeName::Decimal => Some(format!("Decimal({})", str_arg)),
      _ => None
    };
    
    match str_type {
      Some(s) => {
        let r = self.arrays.iter().fold(s, |acc, arr| {
          // FIXME: not sure
          format!("Array<Arc<{}>>", acc)
        });

        Some(r)
      },
      None => None
    }
  }
}