use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

use crate::ast::enums::EnumBlock;
use crate::ast::project::ProjectBlock;
use crate::ast::refs::RefBlock;
use crate::ast::table::TableBlock;
use crate::ast::table_group::TableGroupBlock;
use crate::ast::schema::SchemaBlock;

#[derive(Parser)]
#[grammar = "src/dbml.pest"]
struct DbmlParser;

pub fn parse(input: &str) -> Result<SchemaBlock, Error<Rule>> {
  let pairs = DbmlParser::parse(Rule::schema, input)?;

  for pair in pairs {
    match pair.as_rule() {
      Rule::schema => {
        return Ok(parse_schema(pair));
      },
      _ => unreachable!("'{:?}' not supposed to get there (schema)!", pair.as_rule()),
    }
  }

  unreachable!("unhandled parsing error!");
}

fn parse_schema(pair: Pair<Rule>) -> SchemaBlock {
  let mut out = SchemaBlock::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::project_decl => out.project = Some(parse_project_decl(p)),
      Rule::table_decl => out.tables.push(parse_table_decl(p)),
      Rule::enum_decl => out.enums.push(parse_enum_decl(p)),
      Rule::ref_decl => out.refs.push(parse_ref_decl(p)),
      Rule::table_group_decl => out.table_groups.push(parse_table_group_decl(p)),
      Rule::EOI => (),
      _ => unreachable!("'{:?}' not supposed to get there (top-level declaration)!", p.as_rule()),
    }
  }

  out
}

fn parse_project_decl(pair: Pair<Rule>) -> ProjectBlock {
  let mut out = ProjectBlock::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::ident => out.name = p.as_str().to_string(),
      Rule::project_block => {
        for pi in p.into_inner() {
          match pi.as_rule() {
            Rule::project_stmt => {
              let (key, value) = parse_project_stmt(pi);

              match key.as_str() {
                "database_type" => out.database_type = value,
                _ => unreachable!("'{:?}' is an invalid key in project_block!", key),
              }
            },
            Rule::note_decl => out.note = parse_note_decl(pi),
            _ => unreachable!("'{:?}' not supposed to get there (project_block)!", pi.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (project_decl)!", p.as_rule()),
    }
  }

  out
}

fn parse_project_stmt(pair: Pair<Rule>) -> (String, String) {
  let mut key = String::default();
  let mut value = String::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::project_key => key = p.as_str().to_string(),
      Rule::string_value => value = parse_string_value(p),
      _ => unreachable!("'{:?}' not supposed to get there (project_stmt)!", p.as_rule()),
    }
  }

  (key, value)
}

fn parse_table_decl(pair: Pair<Rule>) -> TableBlock {
  let mut out = TableBlock::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::decl_ident => {
        let (schema, name) = parse_decl_ident(p);
        
        out.id.name = name;
        out.id.schema = schema;
      },
      Rule::table_alias => out.id.alias = Some(p.as_str().to_string()),
      Rule::table_block => {

      },
      _ => unreachable!("'{:?}' not supposed to get there (table_decl)!", p.as_rule()),
    }
  }

  out
}

fn parse_enum_decl(pair: Pair<Rule>) -> EnumBlock {
  let mut out = EnumBlock::default();
  
  for p in pair.into_inner() {
    
  }

  out
}

fn parse_ref_decl(pair: Pair<Rule>) -> RefBlock {
  let mut out = RefBlock::default();
  
  for p in pair.into_inner() {
    
  }

  out
}

fn parse_table_group_decl(pair: Pair<Rule>) -> TableGroupBlock {
  let mut out = TableGroupBlock::default();
  
  for p in pair.into_inner() {
    
  }

  out
}

fn parse_note_decl(pair: Pair<Rule>) -> String {
  let mut out = String::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::note_short | Rule::note_block => {
        for pi in p.into_inner() {
          match pi.as_rule() {
            Rule::string_value => out = parse_string_value(pi),
            _ => unreachable!("'{:?}' not supposed to get there (note_short | note_block)!", pi.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (note_decl)!", p.as_rule()),
    }
  }

  out
}

fn parse_string_value(pair: Pair<Rule>) -> String {
  let mut out = String::default();
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::triple_quoted_string => {
        for pi in p.into_inner() {
          match pi.as_rule() {
            Rule::triple_quoted_value => out = pi.as_str().to_string(),
            _ => unreachable!("'{:?}' not supposed to get there (triple_quoted_string)!", pi.as_rule()),
          }
        }
      },
      Rule::single_quoted_string => {
        for pi in p.into_inner() {
          match pi.as_rule() {
            Rule::single_quoted_value => out = pi.as_str().to_string(),
            _ => unreachable!("'{:?}' not supposed to get there (single_quoted_string)!", pi.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (string_value)!", p.as_rule()),
    }
  }

  out
}

fn parse_decl_ident(pair: Pair<Rule>) -> (Option<String>, String) {
  let mut schema = None;
  let mut name = String::default();
  let mut tokens = vec![];
  
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::ident => tokens.push(p.as_str().to_string()),
      _ => unreachable!("'{:?}' not supposed to get there (decl_indent)!", p.as_rule()),
    }
  }

  if tokens.len() == 2 {
    schema = Some(tokens.remove(0));
    name = tokens.remove(0);
  } else if tokens.len() == 1 {
    name = tokens.remove(0);
  } else {
    unreachable!("unwell formatted ident!")
  }

  (schema, name)
}