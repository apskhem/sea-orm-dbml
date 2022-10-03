use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

use crate::ast::enums::{EnumBlock, EnumValue};
use crate::ast::project::ProjectBlock;
use crate::ast::refs::{RefBlock, RefId, Relation};
use crate::ast::table::{TableBlock, TableField, ColumnType, Value, ColumnSettings, TableId};
use crate::ast::table_group::{TableGroupBlock, TableGroupId};
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
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::project_decl => out.project = Some(parse_project_decl(p1)),
      Rule::table_decl => out.tables.push(parse_table_decl(p1)),
      Rule::enum_decl => out.enums.push(parse_enum_decl(p1)),
      Rule::ref_decl => out.refs.push(parse_ref_decl(p1)),
      Rule::table_group_decl => out.table_groups.push(parse_table_group_decl(p1)),
      Rule::EOI => (),
      _ => unreachable!("'{:?}' not supposed to get there (top-level declaration)!", p1.as_rule()),
    }
  }

  out
}

fn parse_project_decl(pair: Pair<Rule>) -> ProjectBlock {
  let mut out = ProjectBlock::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => out.name = p1.as_str().to_string(),
      Rule::project_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::project_stmt => {
              let (key, value) = parse_project_stmt(p2);

              match key.as_str() {
                "database_type" => out.database_type = value,
                _ => unreachable!("'{:?}' is an invalid key in project_block!", key),
              }
            },
            Rule::note_decl => out.note = parse_note_decl(p2),
            _ => unreachable!("'{:?}' not supposed to get there (project_block)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (project_decl)!", p1.as_rule()),
    }
  }

  out
}

fn parse_project_stmt(pair: Pair<Rule>) -> (String, String) {
  let mut key = String::default();
  let mut value = String::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::project_key => key = p1.as_str().to_string(),
      Rule::string_value => value = parse_string_value(p1),
      _ => unreachable!("'{:?}' not supposed to get there (project_stmt)!", p1.as_rule()),
    }
  }

  (key, value)
}

fn parse_table_decl(pair: Pair<Rule>) -> TableBlock {
  let mut out = TableBlock::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::decl_ident => {
        let (schema, name) = parse_decl_ident(p1);
        
        out.id.name = name;
        out.id.schema = schema;
      },
      Rule::table_alias => {
        out.id.alias = Some(p1.as_str().to_string())
      },
      Rule::table_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::table_col => {
              let field = parse_table_col(p2);

              out.fields.push(field)
            },
            Rule::note_decl => {
              out.note = parse_note_decl(p2)
            },
            _ => unreachable!("'{:?}' not supposed to get there (table_block)!", p2.as_rule()),
          }
        }
      }
      _ => unreachable!("'{:?}' not supposed to get there (table_decl)!", p1.as_rule()),
    }
  }

  out
}

fn parse_table_col(pair: Pair<Rule>) -> TableField {
  let mut out = TableField::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        out.col_name = p1.as_str().to_string();
      },
      Rule::col_type => {
        let (col_type, col_args, is_array) = parse_col_type(p1);

        out.col_settings.is_array = is_array;
        out.col_args = col_args;
        out.col_type = col_type;
      },
      Rule::col_settings => {
        out.col_settings = parse_col_settings(p1)
      },
      _ => unreachable!("'{:?}' not supposed to get there (table_col)!", p1.as_rule()),
    }
  }

  out
}

fn parse_col_type(pair: Pair<Rule>) -> (ColumnType, Vec<Value>, bool) {
  let mut is_array = false;
  let mut col_args = vec![];
  let mut col_type = ColumnType::Undef;

  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::col_type_single | Rule::col_type_array => {
        is_array = p1.as_rule() == Rule::col_type_array;

        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::var => {
              col_type = ColumnType::Raw(p2.as_str().to_string())
            },
            Rule::col_type_arg => {
              col_args = parse_col_type_arg(p2)
            },
            _ => unreachable!("'{:?}' not supposed to get there (col_type_single | col_type_array)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (project_block)!", p1.as_rule()),
    }
  }

  (col_type, col_args, is_array)
}

fn parse_col_type_arg(pair: Pair<Rule>) -> Vec<Value> {
  let mut out = vec![];
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::value => {
        out.push(parse_value(p1))
      },
      _ => unreachable!("'{:?}' not supposed to get there (col_type_arg)!", p1.as_rule()),
    }
  }

  out
}

fn parse_col_settings(pair: Pair<Rule>) -> ColumnSettings {
  let mut out = ColumnSettings::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::col_attribute => {
        match p1.as_str() {
          "unique" => out.is_unique = true,
          "primary key" | "pk" => out.is_pk = true,
          "null" => out.is_nullable = true,
          "not null" => (),
          "increment" => out.is_incremental = true,
          _ => {
            for p2 in p1.into_inner() {
              match p2.as_rule() {
                Rule::col_default => {
                  out.default = Some(parse_value(p2))
                },
                Rule::note_inline => {
                  out.note = parse_note_inline(p2)
                },
                Rule::ref_inline => {
                  out.refs.push(parse_ref_stmt_inline(p2))
                },
                _ => unreachable!("'{:?}' not supposed to get there (col_attribute)!", p2.as_rule()),
              }
            }
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (col_settings)!", p1.as_rule()),
    }
  }

  out
}

fn parse_enum_decl(pair: Pair<Rule>) -> EnumBlock {
  let mut out = EnumBlock::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::decl_ident => {
        let (schema, name) = parse_decl_ident(p1);
        
        out.id.name = name;
        out.id.schema = schema;
      },
      Rule::enum_block => {
        out.values = parse_enum_block(p1)
      },
      _ => unreachable!("'{:?}' not supposed to get there (enum_decl)!", p1.as_rule()),
    }
  }

  out
}

fn parse_enum_block(pair: Pair<Rule>) -> Vec<EnumValue> {
  let mut out = vec![];
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::enum_value => {
        out.push(parse_enum_value(p1))
      },
      _ => unreachable!("'{:?}' not supposed to get there (enum_block)!", p1.as_rule()),
    }
  }

  out
}

fn parse_enum_value(pair: Pair<Rule>) -> EnumValue {
  let mut out = EnumValue::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        out.value = p1.as_str().to_string()
      },
      Rule::enum_settings => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::enum_attribute => {
              for p3 in p2.into_inner() {
                match p3.as_rule() {
                  Rule::note_inline => {
                    out.note = parse_note_inline(p3)
                  },
                  _ => unreachable!("'{:?}' not supposed to get there (enum_attribute)!", p3.as_rule()),
                }
              }
            },
            _ => unreachable!("'{:?}' not supposed to get there (enum_settings)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (enum_value)!", p1.as_rule()),
    }
  }

  out
}

fn parse_ref_decl(pair: Pair<Rule>) -> RefBlock {
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ref_block | Rule::ref_short => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ref_stmt => {
              return parse_ref_stmt_inline(p2)
            },
            _ => unreachable!("'{:?}' not supposed to get there (ref_block | ref_short)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (ref_decl)!", p1.as_rule()),
    }
  }

  unreachable!("something went wrong parsing ref_decl!")
}

fn parse_ref_stmt_inline(pair: Pair<Rule>) -> RefBlock {
  let mut out = RefBlock::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::relation => {
        out.rel = match p1.as_str() {
          "<" => Relation::One2Many,
          ">" => Relation::Many2One,
          "-" => Relation::One2One,
          "<>" => Relation::Many2Many,
          _ => unreachable!("'{:?}' is not a correct relation symbol!", p1.as_rule()),
        }
      },
      Rule::ref_ident => {
        let value = parse_ref_ident(p1);

        if out.rel == Relation::Undef {
          out.lhs = Some(value);
        } else {
          out.rhs = value;
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (ref_stmt | ref_inline)!", p1.as_rule()),
    }
  }

  out
}

fn parse_ref_ident(pair: Pair<Rule>) -> RefId {
  let mut out = RefId::default();
  let mut tmp_tokens = vec![];
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        tmp_tokens.push(p1.as_str().to_string())
      },
      Rule::ref_composition => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::ident => {
              out.compositions.push(p2.as_str().to_string())
            },
            _ => unreachable!("'{:?}' not supposed to get there (ref_composition)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (ref_indent)!", p1.as_rule()),
    }
  }

  if tmp_tokens.len() == 2 {
    out.schema = Some(tmp_tokens.remove(0));
    out.table = tmp_tokens.remove(0);
  } else if tmp_tokens.len() == 1 {
    out.table = tmp_tokens.remove(0);
  } else {
    unreachable!("unwell formatted ident!")
  }

  out
}

fn parse_table_group_decl(pair: Pair<Rule>) -> TableGroupBlock {
  let mut out = TableGroupBlock::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => {
        out.name = p1.as_str().to_string()
      },
      Rule::table_group_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::decl_ident => {
              let (schema, name) = parse_decl_ident(p2);
              
              let value = TableGroupId {
                name_1: schema,
                name_2: name,
              };

              out.table_ids.push(value)
            },
            _ => unreachable!("'{:?}' not supposed to get there (table_group_block)!", p2.as_rule()),
          }
        }
      }
      _ => unreachable!("'{:?}' not supposed to get there (note_decl)!", p1.as_rule()),
    }
  }

  out
}

fn parse_note_decl(pair: Pair<Rule>) -> String {
  let mut out = String::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::note_short | Rule::note_block => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::string_value => {
              out = parse_string_value(p2)
            },
            _ => unreachable!("'{:?}' not supposed to get there (note_short | note_block)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (note_decl)!", p1.as_rule()),
    }
  }

  out
}

fn parse_note_inline(pair: Pair<Rule>) -> String {
  let mut out = String::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::string_value => {
        out = parse_string_value(p1)
      },
      _ => unreachable!("'{:?}' not supposed to get there (note_inline)!", p1.as_rule()),
    }
  }

  out
}

fn parse_string_value(pair: Pair<Rule>) -> String {
  let mut out = String::default();
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::triple_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::triple_quoted_value => {
              out = p2.as_str().to_string()
            },
            _ => unreachable!("'{:?}' not supposed to get there (triple_quoted_string)!", p2.as_rule()),
          }
        }
      },
      Rule::single_quoted_string => {
        for p2 in p1.into_inner() {
          match p2.as_rule() {
            Rule::single_quoted_value => {
              out = p2.as_str().to_string()
            },
            _ => unreachable!("'{:?}' not supposed to get there (single_quoted_string)!", p2.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (string_value)!", p1.as_rule()),
    }
  }

  out
}

fn parse_value(pair: Pair<Rule>) -> Value {
  let mut out = None;
  
  for p2 in pair.into_inner() {
    match p2.as_rule() {
      Rule::string_value => {
        let value = parse_string_value(p2);

        out = Some(Value::String(value));
      },
      Rule::number_value => {
        for p3 in p2.into_inner() {
          match p3.as_rule() {
            Rule::decimal => {
              let value = p3.as_str().parse::<f32>().unwrap();

              out = Some(Value::Decimal(value));
            },
            Rule::integer => {
              let value = p3.as_str().parse::<i32>().unwrap();

              out = Some(Value::Integer(value));
            },
            _ => unreachable!("'{:?}' not supposed to get there (number_value)!", p3.as_rule()),
          }
        }
      },
      Rule::boolean_value => {
        for p3 in p2.into_inner() {
          match p3.as_str() {
            "true" => out = Some(Value::Bool(true)),
            "false" => out = Some(Value::Bool(false)),
            "null" => out = Some(Value::Null),
            _ => unreachable!("'{:?}' not supposed to get there (boolean_value)!", p3.as_rule()),
          }
        }
      },
      _ => unreachable!("'{:?}' not supposed to get there (value)!", p2.as_rule()),
    }
  }

  out.unwrap()
}

fn parse_decl_ident(pair: Pair<Rule>) -> (Option<String>, String) {
  let mut schema = None;
  let mut name = String::default();
  let mut tmp_tokens = vec![];
  
  for p1 in pair.into_inner() {
    match p1.as_rule() {
      Rule::ident => tmp_tokens.push(p1.as_str().to_string()),
      _ => unreachable!("'{:?}' not supposed to get there (decl_indent)!", p1.as_rule()),
    }
  }

  if tmp_tokens.len() == 2 {
    schema = Some(tmp_tokens.remove(0));
    name = tmp_tokens.remove(0);
  } else if tmp_tokens.len() == 1 {
    name = tmp_tokens.remove(0);
  } else {
    unreachable!("unwell formatted ident!")
  }

  (schema, name)
}