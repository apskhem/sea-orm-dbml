use std::io::Result;

use crate::generator::{Block, Codegen};
use crate::{NAME, VERSION};

use inflector::Inflector;

use self::config::*;
use self::traits::*;

use dbml_rs::*;

pub mod config;
pub mod err;
pub mod traits;
pub mod utils;

pub fn compile(ast: analyzer::SemanticSchemaBlock, config: &Config) -> Result<String> {
  let codegen = Codegen::new().line(format!("//! Generated by {NAME} {VERSION}"));

  let codegen = if ast.enums.is_empty() {
    codegen
  } else {
    codegen
      .line_skip(1)
      .line("use sea_orm::entity::prelude::*;")
  };

  let codegen = gen_entity_modules(&ast, codegen, config);
  let codegen = gen_enum_modules(&ast, codegen, config);

  Ok(codegen.to_string())
}

fn gen_entity_modules(ast: &analyzer::SemanticSchemaBlock, codegen: Codegen, config: &Config) -> Codegen {
  ast.tables.clone().iter().fold(codegen, |acc, table| {
    let ast::table::TableBlock {
      ident,
      cols: fields,
      indexes,
      ..
    } = table.clone();

    let table_block = Block::new(2, Some("pub struct Model"));
    let rel_block = Block::new(2, Some("pub enum Relation"));
    let active_model_block = Block::new(2, Some("impl ActiveModelBehavior for ActiveModel"));
    let mut rel_entity_blocks: Vec<_> = vec![];

    // field listing
    let table_block = fields.iter().cloned().fold(table_block, |acc, field| {
      let mut out_fields = vec![];

      if let Some(exp_type) = field.r#type.to_col_type() {
        out_fields.push(format!(r#"column_type = "{}""#, exp_type))
      }
      if field.settings.is_pk {
        out_fields.push(format!("primary_key"));

        if !field.settings.is_incremental {
          out_fields.push(format!("auto_increment = false"))
        }
      } else if table.meta_indexer.pk_list.contains(&field.name) {
        out_fields.push(format!("primary_key"));
      }
      if field.settings.is_nullable {
        out_fields.push(format!("nullable"))
      }
      if table.meta_indexer.indexed_list.contains(&field.name) {
        out_fields.push(format!("indexed"))
      }
      if field.settings.is_unique || table.meta_indexer.unique_list.contains(&field.name) {
        out_fields.push(format!("unique"))
      }
      if let Some(default) = &field.settings.default {
        let default_string = match default {
          ast::table::Value::String(val) => format!(r#""{}""#, val),
          ast::table::Value::Expr(val) => format!(r#""{}""#, val),
          _ => default.to_string(),
        };

        match default {
          ast::table::Value::Expr(_) => {
            out_fields.push(format!(r#"default_expr = {}"#, default_string))
          }
          _ => out_fields.push(format!(r#"default_value = {}"#, default_string)),
        };
      }

      let field_rust_type = field.r#type.to_rust_type();
      let field_string = match field.settings.is_nullable {
        true => format!("Option<{}>", field_rust_type),
        false => field_rust_type,
      };

      acc
        .line_cond(
          !out_fields.is_empty(),
          format!("#[sea_orm({})]", out_fields.join(", ")),
        )
        .line(format!("pub {}: {},", field.name, field_string))
    });

    // relation listing
    let (rto_vec, rby_vec, rself_vec) = ast.get_table_refs(&ident);

    let rel_block = rself_vec.into_iter().fold(rel_block, |acc, rto| {
      let from_field_pascal = rto.lhs.compositions.get(0).unwrap().to_pascal_case();
      let to_field_pascal = rto.rhs.compositions.get(0).unwrap().to_pascal_case();

      let derive = {
        let mut attrs = vec![
          format!(r#"belongs_to = "Entity""#),
          format!(r#"from = "Column::{}""#, from_field_pascal),
          format!(r#"to = "Column::{}""#, to_field_pascal),
        ];

        if let Some(settings) = rto.settings {
          if let Some(action) = settings.on_delete {
            attrs.push(format!(
              r#"on_delete = "{}""#,
              action.to_string().to_pascal_case()
            ))
          }
          if let Some(action) = settings.on_update {
            attrs.push(format!(
              r#"on_update = "{}""#,
              action.to_string().to_pascal_case()
            ))
          }
        }

        format!(r#"#[sea_orm({})]"#, attrs.join(", "))
      };

      rel_entity_blocks.push(Block::new(2, Some("pub struct SelfReferencingLink")));

      rel_entity_blocks.push(
        Block::new(2, Some("impl Linked for SelfReferencingLink"))
          .line("type FromEntity = Entity;")
          .line("type ToEntity = Entity;")
          .line_skip(1)
          .block(
            Block::new(3, Some("fn link(&self) -> Vec<RelationDef>"))
              .line("vec![Relation::SelfReferencing.def()]"),
          ),
      );

      acc.line(derive).line("SelfReferencing,")
    });

    let rel_block = rto_vec.into_iter().fold(rel_block, |acc, rto| {
      let from_field_pascal = rto.lhs.compositions.get(0).unwrap().to_pascal_case();
      let to_field_pascal = rto.rhs.compositions.get(0).unwrap().to_pascal_case();
      let name_pascal = rto.rhs.table.to_pascal_case();
      let name_snake = rto.rhs.table.to_snake_case();

      let derive = match rto.rel {
        ast::refs::Relation::One2One | ast::refs::Relation::Many2One => {
          let mut attrs = vec![
            format!(r#"belongs_to = "super::{}::Entity""#, name_snake),
            format!(r#"from = "Column::{}""#, from_field_pascal),
            format!(
              r#"to = "super::{}::Column::{}""#,
              name_snake, to_field_pascal
            ),
          ];

          if let Some(settings) = rto.settings {
            if let Some(action) = settings.on_delete {
              attrs.push(format!(
                r#"on_delete = "{}""#,
                action.to_string().to_pascal_case()
              ))
            }
            if let Some(action) = settings.on_update {
              attrs.push(format!(
                r#"on_update = "{}""#,
                action.to_string().to_pascal_case()
              ))
            }
          }

          format!(r#"#[sea_orm({})]"#, attrs.join(", "))
        }
        _ => panic!("unsupported_rel"),
      };

      rel_entity_blocks.push(
        Block::new(
          2,
          Some(format!(
            "impl Related<super::{}::Entity> for Entity",
            name_snake
          )),
        )
        .block(
          Block::new(3, Some("fn to() -> RelationDef"))
            .line(format!("Relation::{}.def()", name_pascal)),
        ),
      );

      acc.line(derive).line(format!("{},", name_pascal))
    });

    let rel_block = rby_vec.into_iter().fold(rel_block, |acc, rby| {
      let name_pascal = rby.lhs.table.to_pascal_case();
      let name_snake = rby.lhs.table.to_snake_case();

      let derive = match rby.rel {
        ast::refs::Relation::One2One => {
          format!(r#"#[sea_orm(has_one = "super::{}::Entity")]"#, name_snake)
        }
        ast::refs::Relation::Many2One => {
          format!(r#"#[sea_orm(has_many = "super::{}::Entity")]"#, name_snake)
        }
        _ => panic!("unsupported_rel"),
      };

      rel_entity_blocks.push(
        Block::new(
          2,
          Some(format!(
            "impl Related<super::{}::Entity> for Entity",
            name_snake
          )),
        )
        .block(
          Block::new(3, Some("fn to() -> RelationDef"))
            .line(format!("Relation::{}.def()", name_pascal)),
        ),
      );

      acc.line(derive).line(format!("{},", name_pascal))
    });

    // active model listing
    /* let active_model_item: Vec<_> = fields
    .iter()
    .cloned()
    .filter_map(|field| {
      let timestamp = "Utc::now().naive_utc()";
      if field.settings.note == Some(format!("@updated_at")) {
        Some(format!("{}: Set({}),", field.name, timestamp))
      }
      else if field.settings.default == Some(ast::table::Value::Expr(format!("now()"))) {
        Some(format!("{}: Set(self.{}.take().or_else(|| Some({}))),", field.name, field.name, timestamp))
      }
      else {
        None
      }
    })
    .chain([
      format!("..self")
    ])
    .collect(); */

    // construct mod block
    let mod_block = Block::new(1, Some(format!("pub mod {}", &ident.name.to_snake_case())))
      .line("use sea_orm::entity::prelude::*;")
      .line_skip(1)
      .line(format!(
        "#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]"
      ))
      .line(format!(
        r#"#[sea_orm(table_name = "{}", schema_name = "{}")]"#,
        &ident.name,
        &ident.schema.unwrap_or_else(|| DEFAULT_SCHEMA.into())
      ))
      .block(table_block)
      .line_skip(1)
      .line("#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]")
      .block(rel_block)
      .block_vec(rel_entity_blocks)
      .line_skip(1)
      .line("impl ActiveModelBehavior for ActiveModel {}");

    acc.line_skip(1).block(mod_block)
  })
}

fn gen_enum_modules(ast: &analyzer::SemanticSchemaBlock, codegen: Codegen, config: &Config) -> Codegen {
  let rs_type = match config.enum_type {
    EnumType::Integer => format!("i32"),
    EnumType::String(_) => format!("String")
  };

  let db_type = match config.enum_type {
    EnumType::Integer => format!("Integer"),
    EnumType::String(_) if config.is_native_enum => format!("Enum"),
    EnumType::String(size) => format!("String({:?})", size)
  };
  
  ast.enums.clone().into_iter().fold(codegen, |acc, r#enum| {
    let ast::enums::EnumBlock {
      ident: ast::enums::EnumIdent { name, schema },
      values,
    } = r#enum;

    let enum_block = Block::new(1, Some(format!("pub enum {}", name.to_pascal_case())));

    let enum_block = values.into_iter().enumerate().fold(enum_block, |acc, (i, value)| {
      let value_name = value.value;
      let value = match config.enum_type {
        EnumType::Integer => format!(r#"num_value = {}"#, i),
        EnumType::String(_) => format!(r#"string_value = "{}""#, value_name)
      };

      acc
        .line(format!(r#"#[sea_orm({})]"#, value))
        .line(format!("{},", value_name.to_pascal_case()))
    });

    acc
      .line_skip(1)
      .line("#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]")
      .line(format!(
        r#"#[sea_orm(rs_type = "{}", db_type = "{}", enum_name = "{}", schema_name = "{}")]"#,
        rs_type,
        db_type,
        name,
        schema.unwrap_or_else(|| DEFAULT_SCHEMA.into())
      ))
      .block(enum_block)
  })
}
