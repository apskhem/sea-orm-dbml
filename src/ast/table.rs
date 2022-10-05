use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableBlock {
  pub cols: Vec<TableColumn>,
  pub ident: TableIdent,
  pub note: Option<String>,
  pub indexes: Option<indexes::IndexesBlock>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableColumn {
  pub name: String,
  pub r#type: ColumnType,
  pub args: Vec<Value>,
  pub settings: ColumnSettings,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Integer(i32),
  Decimal(f32),
  Bool(bool),
  Null
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ColumnType {
  /// The initial value (default)
  #[default] Undef,
  /// The type is waiting to be parsed and validated.
  Raw(String),
  Enum(String),
  Char,
  VarChar,
  SmallInt,
  Integer,
  BigInt,
  Real,
  DoublePrecision,
  Bool,
  ByteArray,
  Date,
  Text,
  Time,
  Timestamp,
  Timestampz,
  Uuid,
  Json,
  Decimal
}

impl ColumnType {
  pub fn match_type(value: &str) -> Self {
    match value {
      "char" => Self::Char,
      "varchar" => Self::VarChar,
      "smallint" => Self::SmallInt,
      "int2" => Self::SmallInt,
      "integer" => Self::Integer,
      "int" => Self::Integer,
      "int4" => Self::Integer,
      "bigint" => Self::BigInt,
      "int8" => Self::BigInt,
      "real" => Self::Real,
      "float4" => Self::Real,
      "float8" => Self::DoublePrecision,
      "bool" => Self::Bool,
      "boolean" => Self::Bool,
      "bytea" => Self::ByteArray,
      "date" => Self::Date,
      "text" => Self::Text,
      "time" => Self::Time,
      "timestamp" => Self::Timestamp,
      "timestampz" => Self::Timestampz,
      "uuid" => Self::Uuid,
      "json" => Self::Json,
      "decimal" => Self::Decimal,
      "numeric" => Self::Decimal,
      _ => unreachable!("'{:?}' type is not supported!", value),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ColumnSettings {
  pub is_pk: bool,
  pub is_unique: bool,
  pub is_nullable: bool,
  pub is_incremental: bool,
  pub is_array: bool,
  pub note: Option<String>,
  pub default: Option<Value>,
  pub refs: Vec<refs::RefBlock>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableIdent {
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>
}