use crate::ast::schema::SchemaBlock;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TranspilerConfig {
  out_path: String
}

pub fn transpile(ast: SchemaBlock, config: TranspilerConfig) {

}