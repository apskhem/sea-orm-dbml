#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumBlock {
  pub id: EnumId,
  pub values: Vec<EnumValue>
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumValue {
  pub value: String,
  pub note: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EnumId {
  pub name: String, 
  pub schema: Option<String>
}