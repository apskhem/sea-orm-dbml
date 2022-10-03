use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupBlock {
  pub name: String,
  pub table_ids: Vec<TableGroupId>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableGroupId {
  pub name_1: Option<String>,
  pub name_2: String
}