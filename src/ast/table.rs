use std::ops::Range;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TableBlock {
  pub fields: Vec<TableField>,
  pub id: TableId,
  pub note: String,
  pub indexes: Option<indexes::IndexesBlock>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableField {
  pub col_name: String,
  pub col_type: ColumnType,
  pub col_settings: TableSettings
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ColumnType {
  Char(Option<u32>),
  VarChar(Option<u32>),
  SmallInt,
  Integer,
  BigInt,
  Real,
  DoublePrecision,
  Bool,
  ByteArray,
  Date,
  Time(Option<u32>),
  Timestamp(Option<u32>),
  Timestampz(Option<u32>),
  Uuid,
  Json,
  Decimal(Option<Range<u32>>)
}

/* "char" = Token::ColumnType(ColumnType::Char(None)),
    "varchar" = Token::ColumnType(ColumnType::VarChar(None)),
    "smallint" = Token::ColumnType(ColumnType::SmallInt),
    "int2" = Token::ColumnType(ColumnType::SmallInt),
    "integer" = Token::ColumnType(ColumnType::Integer),
    "int" = Token::ColumnType(ColumnType::Integer),
    "int4" = Token::ColumnType(ColumnType::Integer),
    "bigint" = Token::ColumnType(ColumnType::BigInt),
    "int8" = Token::ColumnType(ColumnType::BigInt),
    "real" = Token::ColumnType(ColumnType::Real),
    "float4" = Token::ColumnType(ColumnType::Real),
    "float8" = Token::ColumnType(ColumnType::DoublePrecision),
    "bool" = Token::ColumnType(ColumnType::Bool),
    "boolean" = Token::ColumnType(ColumnType::Bool),
    "bytea" = Token::ColumnType(ColumnType::ByteArray),
    "date" = Token::ColumnType(ColumnType::Date),
    "time" = Token::ColumnType(ColumnType::Time(None)),
    "timestamp" = Token::ColumnType(ColumnType::Timestamp(None)),
    "timestampz" = Token::ColumnType(ColumnType::Timestampz(None)),
    "uuid" = Token::ColumnType(ColumnType::Uuid),
    "json" = Token::ColumnType(ColumnType::Json),
    "decimal" = Token::ColumnType(ColumnType::Decimal(None)),
    "numeric" = Token::ColumnType(ColumnType::Decimal(None)), */

/* 
"<" = Token::Relation(Relation::One2Many),
    ">" = Token::Relation(Relation::Many2One),
    "-" = Token::Relation(Relation::One2One),
    "<>" = Token::Relation(Relation::Many2Many),
*/

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TableSettings {
  pub is_pk: bool,
  pub is_unique: bool,
  pub is_nullable: bool,
  pub is_autoincrement: bool,
  pub is_array: bool,
  pub note: String,
  pub default: Option<String>,
  pub r#ref: Vec<refs::RefBlock>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TableId {
  pub name: String,
  pub schema: Option<String>,
  pub alias: Option<String>
}