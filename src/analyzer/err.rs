// pub type AnalyzingResult = Result<()>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Err {
  ProjectSettingNotFound,
  TableGroupNotFound,
  NullablePrimaryKey,
  ArrayPrimaryKey,
  DuplicateProjectSetting,
  DuplicatePrimaryKey,
  DuplicateTableName,
  DuplicateRelation,
  DuplicateEnumName,
  DuplicateTableGroupName,
  SchemaNotFound,
  TableNotFound,
  ColumnNotFound,
  EnumNotFound,
  EnumValueNotFound,
  MismatchedForeignKeyType,
}
