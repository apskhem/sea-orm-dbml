use crate::ast::project::*;

pub const SUPPORTED_SEA_ORM_DB_LIST: &'static [DatabaseType] = &[
  DatabaseType::PostgreSQL,
  DatabaseType::MySQL,
  DatabaseType::SQLite
];
