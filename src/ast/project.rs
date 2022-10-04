#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ProjectBlock {
  pub name: String,
  pub database_type: String,
  pub note: Option<String>
}
