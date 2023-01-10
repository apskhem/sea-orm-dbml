pub enum DatabaseType {
  PostgreSQL,
  Oracle,
  MySQL,
  MongoDB,
  MSSQL,
  SQLite,
  MariaDB,
}

pub const SUPPORTED_SEA_ORM_DB_LIST: &'static [DatabaseType] = &[
  DatabaseType::PostgreSQL,
  DatabaseType::MySQL,
  DatabaseType::SQLite
];
