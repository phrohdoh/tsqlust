// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn to_pair(&self) -> (usize, usize) {
        (self.line, self.col)
    }
}

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
pub struct SelectStatement {
    pub top_statement: Option<Node<TopStatement>>,
    pub column_name_list: Node<ColumnNameList>,
}

impl SelectStatement {
    pub fn is_star(&self) -> bool {
        self.column_name_list.value.is_star()
    }
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct TopStatement {
    // TODO: Store this as a Token / Keyword / something other than just a Position
    pub top_keyword_pos: Position,
    pub expr: Node<Expression>,
    pub is_legacy: bool,
}

#[derive(Debug)]
pub struct ColumnNameList {
    pub column_names: Vec<String>,
}

impl ColumnNameList {
    pub fn is_star(&self) -> bool {
        if let Some(name) = self.column_names.get(0) {
            name == "*"
        } else {
            false
        }
    }
}