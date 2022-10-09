use crate::DEFAULT_SCHEMA;
use crate::ast::*;

mod block;
mod err;
mod indexer;

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
  /// Identifier and alias indexer.
  pub indexer: indexer::Indexer,
}

type TableRefTuple = (Vec<block::IndexedRefBlock>, Vec<block::IndexedRefBlock>, Vec<block::IndexedRefBlock>);

impl SematicSchemaBlock {
  /// Gets a table block's relation (ref to, ref by, ref self).
  pub fn get_table_refs(&self, table_ident: &table::TableIdent) -> TableRefTuple {
    let mut ref_to_blocks = vec![];
    let mut ref_by_blocks = vec![];
    let mut ref_self_blocks = vec![];

    let eq = |table_ident: &table::TableIdent, ref_ident: &refs::RefIdent| {
      table_ident.schema == ref_ident.schema && table_ident.name == ref_ident.table
    };

    for ref_block in self.refs.iter() {
      let lhs_ident = self.indexer.refer_ref_alias(&ref_block.lhs);
      let rhs_ident = self.indexer.refer_ref_alias(&ref_block.rhs);

      if eq(&table_ident, &lhs_ident) && eq(&table_ident, &rhs_ident) {
        ref_self_blocks.push(ref_block.clone())
      }
      else if eq(&table_ident, &lhs_ident) {
        ref_to_blocks.push(ref_block.clone())
      }
      else if eq(&table_ident, &rhs_ident) {
        ref_by_blocks.push(ref_block.clone())
      }
    }

    (ref_to_blocks, ref_by_blocks, ref_self_blocks)
  }
}

impl schema::SchemaBlock<'_> {
  pub fn analyze(self) -> SematicSchemaBlock {
    let Self {
      input,
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
    let mut indexer = indexer::Indexer::default();
    let mut indexed_refs: Vec<_> = refs.into_iter().map(block::IndexedRefBlock::from).collect();
    
    indexer.index_table(&tables);
    indexer.index_enums(&enums);
    indexer.index_table_groups(&table_groups);

    // collect refs
    for table in tables.iter() {
      for col in table.cols.iter() {
        let indexed_ref = block::IndexedRefBlock::from_inline(
          col.settings.refs.clone(),
          table.ident.clone(),
          col.name.clone()
        );

        indexed_refs.extend(indexed_ref);
      }
    }

    // validate table type
    let tables = tables.into_iter().map(|table| {
      let cols = table.cols.into_iter().map(|col| {
        let r#type = col.r#type;

        if r#type == table::ColumnType::Undef {
          panic!("undef_table_field")
        }

        let r#type = if let table::ColumnType::Raw(raw) = r#type {
          if let Ok(valid) = table::ColumnType::match_type(&raw) {
            if col.args.is_empty() {
              valid
            } else {
              // validate args (if has)
              match valid {
                table::ColumnType::VarChar | table::ColumnType::Char => {
                  if col.args.len() != 1 {
                    panic!("varchar_incompatible_args")
                  }

                  col.args.iter().fold(valid, |acc, arg| {
                    if let table::Value::Integer(_) = arg {
                      acc
                    } else {
                      panic!("varchar_args_is_not_integer")
                    }
                  })
                },
                table::ColumnType::Decimal => {
                  if col.args.len() != 2 {
                    panic!("decimal_incompatible_args")
                  }

                  col.args.iter().fold(valid, |acc, arg| {
                    if let table::Value::Integer(_) = arg {
                      acc
                    } else {
                      panic!("decimal_args_is_not_integer")
                    }
                  })
                },
                _ => panic!("invalid args usage")
              }
            }
          } else {
            // FIXME: add support for default enum value
            // let values = if let Some(v) = col.settings.default { vec![v] } else { vec![] };
            // TODO: add support for enum with schema
            if let Err(msg) = indexer.lookup_enum_values(&None, &raw, &vec![]) {
              panic!("{}", msg)
            } else {
              table::ColumnType::Enum(raw)
            }
          }
        } else {
          panic!("preprecessing_type_is_not_raw")
        };
        
        table::TableColumn {
          r#type,
          ..col
        }
      }).collect();

      table::TableBlock {
        cols,
        ..table
      }
    }).collect();

    // validate ref
    for indexed_ref in indexed_refs.clone().into_iter() {
      match indexed_ref.rel {
        refs::Relation::One2Many => panic!("one-to-many relation is unsupported"),
        refs::Relation::Many2Many => panic!("many-to-many relation is unsupported"),
        _ => ()
      }

      if let Err(msg) = indexed_ref.validate_ref_type(&tables, &indexer) {
        panic!("{}", msg)
      }

      for r in indexed_refs.iter() {
        if r.lhs.compositions.len() != 1 || r.rhs.compositions.len() != 1 {
          panic!("composite reference is unsupported")
        }
        if r.lhs.compositions.len() != r.rhs.compositions.len() {
          panic!("composite reference must have the same length")
        }
      }

      let count = indexed_refs.iter().fold(0, |acc, other_indexed_ref| {
        if indexed_ref.is_same_lhs_as(&other_indexed_ref, &indexer) {
          acc + 1
        } else {
          acc
        }
      });

      if count != 1 {
        panic!("dedup_relation_decl")
      }
    }

    SematicSchemaBlock {
      project,
      tables,
      table_groups,
      refs: indexed_refs,
      enums,
      indexer,
    }
  }
}