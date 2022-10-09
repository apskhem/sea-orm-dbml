use std::collections::HashMap;

use crate::ast::*;
use crate::DEFAULT_SCHEMA;

use super::indexer;
use super::indexer::IndexedSchemaBlock;

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

  pub fn validate_ref_type(
    &self,
    tables: &Vec<table::TableBlock>,
    indexer: &indexer::Indexer
  ) -> Result<(), String> {
    let refer_alias = |ident: &refs::RefIdent, indexer: &indexer::Indexer| {
      if let Some((schema, table)) = indexer.refer_alias(&ident.table) {
        refs::RefIdent {
          schema: schema.clone(),
          table: table.clone(),
          compositions: ident.compositions.clone()
        }
      } else {
        ident.clone()
      }
    };

    let lhs_ident = refer_alias(&self.lhs, indexer);
    let rhs_ident = refer_alias(&self.rhs, indexer);

    indexer.lookup_table_fields(&lhs_ident.schema, &lhs_ident.table, &lhs_ident.compositions)?;
    indexer.lookup_table_fields(&rhs_ident.schema, &rhs_ident.table, &rhs_ident.compositions)?;

    let lhs_table = tables.iter().find(|table| {
      table.ident.schema == lhs_ident.schema && table.ident.name == lhs_ident.table
    }).unwrap();

    let rhs_table = tables.iter().find(|table| {
      table.ident.schema == rhs_ident.schema && table.ident.name == rhs_ident.table
    }).unwrap();

    let field_pairs = lhs_ident.compositions.iter().zip(rhs_ident.compositions.iter());

    for (l, r) in field_pairs.into_iter() {
      let l_field = lhs_table.cols.iter().find(|col| &col.name == l).unwrap();
      let r_field = rhs_table.cols.iter().find(|col| &col.name == r).unwrap();

      if l_field.r#type != r_field.r#type {
        return Err(format!("reference column type is mismatched"))
      }
    }

    Ok(())
  }

  pub fn is_same_lhs_as(&self, other: &Self, indexer: &indexer::Indexer) -> bool {
    let refer_alias = |ident: &refs::RefIdent, indexer: &indexer::Indexer| {
      if let Some((schema, table)) = indexer.refer_alias(&ident.table) {
        refs::RefIdent {
          schema: schema.clone(),
          table: table.clone(),
          compositions: ident.compositions.clone()
        }
      } else {
        ident.clone()
      }
    };

    let self_ident = refer_alias(&self.lhs, indexer);
    let other_ident = refer_alias(&other.lhs, indexer);

    self_ident == other_ident
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