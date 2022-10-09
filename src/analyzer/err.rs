// pub type AnalyzingResult = Result<()>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Err {
  NullablePrimaryKey,
  ArrayPrimaryKey,
  ConflictedPrimaryKey,
  DuplicatedProjectSetting,
  DuplicatedPrimaryKey,
  DuplicatedTableName,
  DuplicatedRelation,
  DuplicatedEnumName,
  DuplicatedEnumValue,
  DuplicatedTableGroupName,
  ProjectSettingNotFound,
  TableGroupNotFound,
  SchemaNotFound,
  TableNotFound,
  ColumnNotFound,
  EnumNotFound,
  EnumValueNotFound,
  MismatchedForeignKeyType,
  MismatchedCompositeForeignKey,
  UnsupportedSyntax
}
