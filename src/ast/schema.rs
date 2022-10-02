use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SchemaBlock {
  /// Overall description of the project. This is optional. The file must contain one or zero 'Project' block.
  pub project: Option<project::ProjectBlock>,
  /// Table block.
  pub tables: Vec<table::TableBlock>,
  /// TableGroup block.
  pub table_groups: Vec<table_group::TableGroupBlock>,
  /// Ref block.
  pub refs: Vec<refs::RefBlock>,
  /// Enums block.
  pub enums: Vec<enums::EnumBlock>
}

impl SchemaBlock {
  pub fn get_table(&self, name: &str) {
    unimplemented!();
  }

  pub fn get_table_group(&self, name: &str) {
    unimplemented!();
  }

  pub fn get_enum(&self, name: &str) {
    unimplemented!();
  }

  pub fn get_schema(&self, name: &str) {
    unimplemented!();
  }

  pub fn get_block_count(&self) {
    unimplemented!();
  }
}