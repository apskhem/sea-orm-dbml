use super::*;

#[derive(Debug, PartialEq, Clone, Default)]
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

  pub fn transform_sematic(self) -> Self {
    self
  }

  pub fn print(&self) {
    println!("Project:");

    println!("{:?}\n----", self.project);

    println!("Tables:");

    self.tables.iter().for_each(|table| println!("{:?}\n----", table));

    println!("TableGroups:");

    self.table_groups.iter().for_each(|table| println!("{:?}\n----", table));

    println!("Refs:");

    self.refs.iter().for_each(|table| println!("{:?}\n----", table));

    println!("Enums:");

    self.enums.iter().for_each(|table| println!("{:?}\n----", table));
  }
}