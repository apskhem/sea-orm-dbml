#[derive(Debug, PartialEq, Clone)]
pub enum TargetLang {
  Rust
}

#[derive(Debug, PartialEq, Clone)]
pub struct Codegen {
  lang: TargetLang,
  root_block: Block,
}

impl Codegen {
  pub fn new(lang: TargetLang) -> Self {
    let codegen = Self {
      lang,
      root_block: Block::root()
    };

    codegen
  }

  pub fn line(mut self, line_content: impl ToString) -> Self {
    self.root_block = self.root_block.line(line_content);

    self
  }

  pub fn block(mut self, block: Block) -> Self {
    self.root_block = self.root_block.block(block);

    self
  }
}

impl ToString for Codegen {
  fn to_string(&self) -> String {
    self.root_block.to_string()
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Block {
  /// level of the block. 0 is the root block.
  level: usize,
  /// content before the block
  content_before_block: Option<String>,
  /// content inside the block
  content: String
}

impl Block {
  fn root() -> Self {
    Self {
      level: 0,
      content_before_block: None,
      content: String::new()
    }
  }

  pub fn new(level: usize, before_block_content: Option<impl ToString>) -> Self {
    let content = if let Some(impl_string) = before_block_content {
      Some(impl_string.to_string())
    } else {
      None
    };

    Self {
      level,
      content_before_block: content,
      content: String::new()
    }
  }

  pub fn line(mut self, line_content: impl ToString) -> Self {
    let indent = "\t".repeat(self.level);

    let new_content = format!("{}{}", indent, line_content.to_string());

    self.content = format!("{}{}\n", self.content, new_content);

    self
  }

  pub fn block(mut self, block: Block) -> Self {
    self.content = format!("{}{}\n", self.content, block.to_string());

    self
  }
}

impl ToString for Block {
  fn to_string(&self) -> String {
    let out = if self.level == 0 {
      format!("{}\n", self.content.clone())
    } else {
      let block_indent = if self.level == 1 { String::new() } else { "\t".repeat(self.level - 1) };

      let upper_block = if let Some(content_before_block) = self.content_before_block.clone() {
        format!("{}{} {}", block_indent, content_before_block, "{")
      } else {
        format!("{}{}", block_indent, "{")
      };

      let lower_block = format!("{}{}", block_indent, "}");

      format!("{}\n{}{}\n", upper_block, self.content.clone(), lower_block)
    };
    
    out
  }
}