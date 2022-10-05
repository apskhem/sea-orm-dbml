// pub type AnalyzingResult = Result<()>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Err {
  ProjectSettingNotFound,
  RefActionNotFound,
  TableGroupNotFound,
  DuplicateProjectSetting,
  NullablePrimaryKey,
  ArrayPrimaryKey,
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
