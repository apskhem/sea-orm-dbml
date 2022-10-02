#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefBlock {
  pub rel: Option<Relation>,
  pub from: Option<RefId>,
  pub to: RefId
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Relation {
  One2One,
  One2Many,
  Many2One,
  Many2Many
}


#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RefId {
  pub schema: Option<String>,
  pub table: String,
  pub field: String,
}