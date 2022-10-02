#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesBlock {
  pub fields: Vec<IndexesField>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesField {
  pub ids: Vec<String>,
  pub settings: Option<IndexesSettings>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexesSettings {
  pub r#type: Option<String>,
  pub name: Option<String>,
  pub is_unique: bool,
  pub is_pk: bool,
}
