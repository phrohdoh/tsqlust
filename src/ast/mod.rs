// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(Debug)]
pub struct Node<TNode> {
    pub pos: Position,
    pub value: TNode,
}

#[derive(Debug)]
/// A [`SELECT`](https://msdn.microsoft.com/en-us/library/ms189499.aspx) statement
pub struct SelectStatement {
    pub top_statement: Option<Node<TopStatement>>,
    pub column_name_list: Node<ColumnNameList>,
}

impl SelectStatement {
    /// Whether or not this `SELECT`'s column name list is a wildcard only
    pub fn is_star(&self) -> bool {
        self.column_name_list.value.is_star()
    }
}

#[derive(PartialEq, Debug)]
/// Represents a literal value (not a variable) found in the query source.
pub enum Literal {
    Bool(bool),
    Int(i32),
    Float(f32),
    Str(String),
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Literal {
        lit: Literal,
    },
}

#[derive(PartialEq, Debug)]
pub enum Token {
    ParenOpen,
    ParenClose,
}

#[derive(Debug)]
/// A [`TOP`](https://msdn.microsoft.com/en-us/library/ms189463.aspx) statement
pub struct TopStatement {
    /// TODO: Store this as a Token / Keyword / something other than just a Position
    pub top_keyword_pos: Position,
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

#[derive(Debug)]
pub struct ColumnNameList {
    pub column_names: Vec<String>,
}

impl ColumnNameList {
    /// Whether or not this is a wildcard-only column name list
    pub fn is_star(&self) -> bool {
        if let Some(name) = self.column_names.get(0) {
            name == "*"
        } else {
            false
        }
    }
}
