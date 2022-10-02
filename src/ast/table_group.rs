use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TableGroupBlock {
  pub table_ids: Vec<table::TableId>
}