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

    // check project block
    if let Some(project_block) = &project {

    } else {
      panic!("no_project_block_found");
    }

    // collect tables
    let mut indexed_refs: Vec<block::IndexedRefBlock> = refs.into_iter().map(block::IndexedRefBlock::from).collect();
    let mut table_group_map = HashMap::new();
    let mut schema_map = HashMap::<String, IndexedSchemaBlock>::new();
    let mut alias_map = HashMap::new();

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
            panic!("pk_dup");
          } else {
            is_pk_passed = true;
          }

          if col.settings.is_nullable {
            panic!("nullable_pk");
          }
          if col.settings.is_array {
            panic!("array_pk");
          }
        }

        let indexed_ref = block::IndexedRefBlock::from_inline(
          col.settings.refs.clone(),
          table.ident.clone(),
          col.name.clone()
        );

        indexed_refs.extend(indexed_ref);

        if let Some(dup_col_name) = col_sets.get(&col.name) {
          panic!("col_name_dup");
        } else {
          col_sets.insert(col.name.clone());
        }
      }

      if let Some(index_block) = schema_map.get_mut(&schema_name) {
        index_block.table_map.insert(name.clone(), col_sets);

        if let Some(alias) = alias {
          if let Some(dup_alias) = alias_map.get(&alias) {
            panic!("alias_name_dup");
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
          panic!("val_dup");
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
    for table_group in table_groups.clone().into_iter() {
      for table in table_group.table_idents.into_iter() {
        let schema_name = table.schema.unwrap_or_else(|| DEFAULT_SCHEMA.into());
        let ident_alias = table.ident_alias;

        let ident = if let Some(ident) = alias_map.get(&ident_alias) {
          if !schema_name.eq("public") {
            panic!("alias_must_not_followed_by_schema")
          }

          ident.1.clone()
        } else {
          ident_alias
        };

        if let Some(index_block) = schema_map.get(&schema_name) {
          if !index_block.table_map.contains_key(&ident) {
            panic!("table_not_found");
          }
        } else {
          panic!("schema_not_found");
        }
      }

      // TODO: add table inside table_group
      table_group_map.insert(table_group.name.clone(), HashSet::new());
    }

    println!("schema: {:?}\n", &schema_map);
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