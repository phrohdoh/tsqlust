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
    pub node: TNode,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub top_statement: Option<TopStatement>,
    pub column_name_list: Vec<String>,
}

impl SelectStatement {
    pub fn is_star(&self) -> bool {
        unimplemented!()
        // self.column_name_list.len() == 1 && self.column_name_list[0] == "*"
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
        lit: self::Literal,
    },
}

#[derive(Debug)]
pub struct TopStatement {
    pub expr: Node<Expression>,
    pub is_legacy: bool,
}
