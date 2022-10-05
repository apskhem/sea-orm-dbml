use std::collections::{HashSet, HashMap};

use crate::DEFAULT_SCHEMA;
use crate::ast::*;

mod err;
mod block;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct IndexedSchemaBlock {
  table_map: HashMap<String, HashSet<String>>,
  enum_map: HashMap<String, HashSet<String>>
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SematicSchemaBlock {
  /// Overall description of the project. This is optional. The file must contain one or zero 'Project' block.
  pub project: Option<project::ProjectBlock>,
  /// Table block.
  pub tables: Vec<table::TableBlock>,
  /// TableGroup block.
  pub table_groups: Vec<table_group::TableGroupBlock>,
  /// Ref block.
  pub refs: Vec<block::IndexedRefBlock>,
  /// Enums block.
  pub enums: Vec<enums::EnumBlock>,
  /// Indexed table groups map.
  pub table_group_map: HashMap<String, HashSet<(Option<String>, String)>>,
  /// Indexed schema map.
  pub schema_map: HashMap<String, IndexedSchemaBlock>,
  /// Indexed alias map.
  pub alias_map: HashMap<String, (Option<String>, String)>
}

impl schema::SchemaBlock {
  pub fn analyze(self) -> SematicSchemaBlock {
    let Self {
      project,
      tables,
      table_groups,
      refs,
      enums,
    } = self;

    // collect tables
    let mut indexed_refs: Vec<block::IndexedRefBlock> = refs.into_iter().map(block::IndexedRefBlock::from).collect();
    let mut table_group_map = HashMap::new();
    let mut schema_map = HashMap::<String, IndexedSchemaBlock>::new();
    let mut alias_map = HashMap::new();

    let mut tmp_refs: Vec<refs::RefBlock> = vec![];

    for table in tables.iter() {
      let table::TableIdent {
        schema,
        name,
        alias
      } = table.ident.clone();

      let schema_name = schema.clone().unwrap_or_else(|| DEFAULT_SCHEMA.into());
      let mut col_sets = HashSet::new();

      let mut is_pk_passed = false;
      for col in table.cols.iter() {
        if col.settings.is_pk {
          if is_pk_passed {
            // TODO: handle pk_dup exception
          } else {
            is_pk_passed = true;
          }

          if col.settings.is_nullable {
            // TODO: handle nullable_pk exception
          }
          if col.settings.is_array {
            // TODO: handle array_pk exception
          }
        }

        let indexed_ref = block::IndexedRefBlock::from_inline(
          col.settings.refs.clone(),
          table.ident.clone(),
          col.name.clone()
        );

        indexed_refs.extend(indexed_ref);

        if let Some(dup_col_name) = col_sets.get(&col.name) {
          // TODO: handle col_name_dup exception
        } else {
          col_sets.insert(col.name.clone());
        }
      }

      if let Some(index_block) = schema_map.get_mut(&schema_name) {
        index_block.table_map.insert(name.clone(), col_sets);

        if let Some(alias) = alias {
          if let Some(dup_alias) = alias_map.get(&alias) {
            // TODO: handle alias_name_dup exception
          } else {
            alias_map.insert(alias.clone(), (schema.clone(), name.clone()));
          }
        }
      } else {
        let mut index_block = IndexedSchemaBlock::default();

        index_block.table_map.insert(name.clone(), col_sets);

        if let Some(alias) = alias {
          alias_map.insert(alias.clone(), (schema.clone(), name.clone()));
        }

        schema_map.insert(schema_name, index_block);
      }
    }

    // collect enums
    for r#enum in enums.iter() {
      let enums::EnumIdent {
        schema,
        name,
      } = r#enum.ident.clone();

      let schema_name = schema.clone().unwrap_or_else(|| DEFAULT_SCHEMA.into());
      let mut value_sets = HashSet::new();

      for value in r#enum.values.iter() {
        if let Some(dup_col_name) = value_sets.get(&value.value) {
          // TODO: handle val_dup exception
        } else {
          value_sets.insert(value.value.clone());
        }
      }

      if let Some(index_block) = schema_map.get_mut(&schema_name) {
        index_block.enum_map.insert(name.clone(), value_sets);
      } else {
        let mut index_block = IndexedSchemaBlock::default();

        index_block.enum_map.insert(name.clone(), value_sets);

        schema_map.insert(schema_name, index_block);
      }
    }

    // collect table_group
    for table_group in table_groups.iter() {

    }

    println!("schema: {:?}", &schema_map);
    println!("indexed_refs: {:?}", &indexed_refs);

    SematicSchemaBlock {
      project,
      tables,
      table_groups,
      refs: indexed_refs,
      enums,
      table_group_map,
      schema_map,
      alias_map
    }
  }
}