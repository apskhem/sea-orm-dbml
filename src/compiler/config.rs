use std::ffi::OsString;

#[derive(Debug)]
pub enum EnumType {
  String(Option<u32>),
  Integer,
}

#[derive(Debug)]
/// Configuration options for the code generation.
pub struct Config {
  /// Input file path.
  pub in_path: OsString,
  /// Output file path (optional). The default output path is `OUT_DIR`.
  pub out_path: OsString,
  /// Database entity target.
  pub target: Target,
  /// Enum type for storing a value. It can be either `String` and `Interger`.
  /// The `String` type requires to specify the length of characters that will be stored.
  pub enum_type: EnumType,
  /// Enable native enum for the database.
  pub is_native_enum: bool
}

impl Default for Config {
  fn default() -> Self {
    Self {
      in_path: OsString::from(""),
      out_path: OsString::from(""),
      target: Target::Postgres,
      enum_type: EnumType::String(None),
      is_native_enum: true
    }
  }
}

impl Config {
  pub fn validate(&self) -> Option<&str> {
    if self.in_path.is_empty() {
      Some("in_path is not set")
    } else if self.out_path.is_empty() {
      Some("out_path is not set")
    } else {
      None
    }
  }
}

/// Database entity target.
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
  // MySQL,
  Postgres,
  // Sqlite
}

