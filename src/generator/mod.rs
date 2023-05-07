#[derive(Debug, PartialEq, Clone)]
pub struct Codegen {
  root_block: Block,
}

impl Codegen {
  pub fn new() -> Self {
    let codegen = Self {
      root_block: Block::root(),
    };

    codegen
  }

  pub fn line(mut self, line_content: impl ToString) -> Self {
    self.root_block = self.root_block.line(line_content);

    self
  }

  pub fn line_cond(mut self, cond: bool, line_content: impl ToString) -> Self {
    self.root_block = self.root_block.line_cond(cond, line_content);

    self
  }

  pub fn line_skip(mut self, line_count: usize) -> Self {
    self.root_block = self.root_block.line_skip(line_count);

    self
  }

  pub fn block(mut self, block: Block) -> Self {
    self.root_block = self.root_block.block(block);

    self
  }

  pub fn block_vec(mut self, block_vec: Vec<Block>) -> Self {
    self.root_block = self.root_block.block_vec(block_vec);

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
  content: String,
}

impl Block {
  fn root() -> Self {
    Self {
      level: 0,
      content_before_block: None,
      content: String::new(),
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
      content: String::new(),
    }
  }

  /// Inserts a single line with the given text content.
  pub fn line(mut self, line_content: impl ToString) -> Self {
    let indent = "\t".repeat(self.level);

    let new_content = format!("{}{}", indent, line_content.to_string());

    self.content = format!("{}{}\n", self.content, new_content);

    self
  }

  /// Inserts a single line when the condition is `true`.
  pub fn line_cond(mut self, cond: bool, line_content: impl ToString) -> Self {
    if !cond {
      return self;
    }

    let indent = "\t".repeat(self.level);

    let new_content = format!("{}{}", indent, line_content.to_string());

    self.content = format!("{}{}\n", self.content, new_content);

    self
  }

  /// Inserts empty lines for the given count.
  pub fn line_skip(mut self, line_count: usize) -> Self {
    let line_string = "\n".repeat(line_count);

    self.content = format!("{}{}", self.content, line_string);

    self
  }

  /// Inserts a block to the scope.
  pub fn block(mut self, block: Block) -> Self {
    self.content = format!("{}{}\n", self.content, block.to_string());

    self
  }

  /// Inserts a vector of blocks to the scope.
  pub fn block_vec(mut self, block_vec: Vec<Block>) -> Self {
    for block in block_vec.into_iter() {
      self.content = format!("{}\n{}\n", self.content, block.to_string());
    }

    self
  }
}

impl ToString for Block {
  fn to_string(&self) -> String {
    let out = if self.level == 0 {
      format!("{}", self.content.clone())
    } else {
      let block_indent = if self.level == 1 {
        String::new()
      } else {
        "\t".repeat(self.level - 1)
      };

      let upper_block = if let Some(content_before_block) = self.content_before_block.clone() {
        format!("{}{} {}", block_indent, content_before_block, "{")
      } else {
        format!("{}{}", block_indent, "{")
      };

      if self.content.is_empty() {
        format!("{}{}", upper_block, "}")
      } else {
        let lower_block = format!("{}{}", block_indent, "}");

        format!("{}\n{}{}", upper_block, self.content.clone(), lower_block)
      }
    };

    out
  }
}
