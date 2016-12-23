// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#[derive(PartialEq, Copy, Clone, Debug, Serialize)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    /// Creates a tuple containing `line, col`.
    pub fn to_pair(&self) -> (usize, usize) {
        (self.line, self.col)
    }
}

/// The default is `{ line: 1, col: 1 }`.
impl Default for Position {
    fn default() -> Position {
        Position { line: 1, col: 1 }
    }
}

impl From<(usize, usize)> for Position {
    fn from((line, col): (usize, usize)) -> Position {
        Position {
            line: line,
            col: col,
        }
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub struct Node<TNode> {
    /// Position in the AST
    ///
    /// This is useful for error reporting.
    pub pos: Position,

    /// The actual node in the tree.
    ///
    /// For example: `SelectStatement`, `CreateTableStatement`, etc.
    pub tnode: TNode,
}

#[derive(PartialEq, Debug, Serialize)]
/// A [`SELECT`](https://msdn.microsoft.com/en-us/library/ms189499.aspx) statement
pub struct SelectStatement {
    pub top_statement: Option<Node<TopStatement>>,
    pub column_name_list: Node<ColumnNameList>,
    pub table_identifier: Node<Identifier>,
}

#[derive(PartialEq, Debug, Serialize)]
/// Represents a literal value (not a variable) found in the query source.
pub enum Literal {
    Bool(bool),
    Int(i32),
    Float(f32),
    Str(String),
}

#[derive(PartialEq, Debug, Serialize)]
/// A table or column name
pub struct Identifier {
    pub value: String,
}

#[derive(PartialEq, Debug, Serialize)]
pub enum Expression {
    Literal {
        lit: Literal,
    },
}

#[derive(PartialEq, Debug, Serialize)]
pub enum Keyword {
    Top
}

#[derive(PartialEq, Debug, Serialize)]
pub enum Token {
    ParenOpen,
    ParenClose,
}

#[derive(PartialEq, Debug, Serialize)]
/// A [`TOP`](https://msdn.microsoft.com/en-us/library/ms189463.aspx) statement
pub struct TopStatement {
    pub top_keyword: Node<Keyword>,
    pub expr: Node<Expression>,

    pub paren_open: Option<Node<Token>>,
    pub paren_close: Option<Node<Token>>,
}

impl TopStatement {
    /// Indicates whether or not this is a legacy statement.
    ///
    /// Statements without parentheses are legacy.
    ///
    /// Legacy: `TOP 10`
    ///
    /// Non-legacy: `TOP (10)`
    pub fn is_legacy(&self) -> bool {
        // We will fail to build the grammar (and therefore this node)
        // if one of these exists but the other doesn't
        // so either missing is indicitive of a legacy TOP statement.
        self.paren_close.is_none() || self.paren_open.is_none()
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub struct ColumnNameList {
    pub identifiers: Vec<Node<Identifier>>,
}

#[derive(PartialEq, Debug, Serialize)]
pub struct CreateTableStatement {
    pub table_identifier: Node<Identifier>,
}
