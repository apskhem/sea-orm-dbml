use crate::ast::*;

/// A validated reference block.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IndexedRefBlock {
  pub rel: refs::Relation,
  pub lhs: refs::RefIdent,
  pub rhs: refs::RefIdent,
  pub settings: Option<refs::RelationSettings>
}

impl IndexedRefBlock {
  pub fn from_inline(ref_blocks: Vec<refs::RefBlock>, table_ident: table::TableIdent, col_name: String) -> Vec<Self> {
    ref_blocks.into_iter().map(|ref_block| {
      let table_ident = table_ident.clone();
      let col_name = col_name.clone();

      let refs::RefBlock {
        rel,
        rhs,
        settings,
        ..
      } = ref_block;

      // TODO: handle the unwrap
  
      let lhs = refs::RefIdent {
        schema: table_ident.schema,
        table: table_ident.name,
        compositions: vec![col_name]
      };
      
      Self {
        rel,
        lhs,
        rhs,
        settings
      }.normalize()
    }).collect()
  }

  fn normalize(self) -> Self {
    // TODO:
    self
  }
}

impl From<refs::RefBlock> for IndexedRefBlock {
  fn from(ref_block: refs::RefBlock) -> Self {
    let refs::RefBlock {
      rel,
      lhs,
      rhs,
      settings
    } = ref_block;

    // TODO: handle the unwrap

    Self {
      rel,
      lhs: lhs.unwrap(),
      rhs,
      settings
    }.normalize()
  }
}