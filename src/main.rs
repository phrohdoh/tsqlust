#![recursion_limit = "200"]

#[macro_use]
extern crate pest;

use pest::prelude::*;

mod ast;

impl_rdp! {
    grammar! {
        tsql = _{ (
            stmt_select
        ) ~ eoi }

        stmt_top_legacy = { kw_top ~ lit_integer ~ kw_percent? }
        stmt_top = { kw_top ~ tok_paren_open ~ expr ~ tok_paren_close ~ kw_percent? ~ kw_with_ties? }

        stmt_select = {
            kw_select
            ~ (stmt_top | stmt_top_legacy)?
            ~ column_name_list ~ kw_from ~ term_id
            ~ clause_where?
        }

        expr = {
            lit_integer
            | expr_add
            | expr_subt
            | expr_mult
            | expr_div
        }

        expr_add = { expr ~ tok_plus ~ expr }
        expr_subt = { expr ~ tok_minus ~ expr }
        expr_mult = { expr ~ tok_star ~ expr }
        expr_div = { expr ~ tok_slash_forward ~ expr }

        lit_bool = { [i"TRUE"] | [i"FALSE"] }
        lit_integer = @{ ['0'..'9']+ }

        literal = {
            lit_bool
            | lit_integer
        }

        term_id = @{ (
            ['a'..'z'] |
            ['A'..'Z'] |
            ['0'..'9'] |
            ["_"]
        )+ }

        tok_plus = { ["+"] }
        tok_minus = { ["-"] }
        tok_ampersand = { ["&"] }
        tok_percent = { ["%"] }
        tok_pipe = { ["|"] }
        tok_caret = { ["^"] }
        tok_star = { ["*"] }
        tok_slash_forward = { ["/"] }
        tok_paren_open = { ["("] }
        tok_paren_close = { [")"] }

        tok_eq = @{ ["="] }
//  tok_eq_eq = { tok_eq ~ tok_eq } // TODO: This is valid somewhere, where?
        tok_bang = @{ ["!"] }
        tok_angle_open = @{ ["<"] }
        tok_angle_close = @{ [">"] }

        op_cmp = {
            op_cmp_eq
//  | op_cmp_eq_eq
            | op_cmp_neq_bang
            | op_cmp_lt
            | op_cmp_gt
            | op_cmp_lt_eq
            | op_cmp_gt_eq
        }

        op_cmp_eq = { tok_eq }
// op_cmp_eq_eq = { tok_eq ~ tok_eq }
        op_cmp_neq_bang = { tok_bang ~ tok_eq }
        op_cmp_lt = { tok_angle_open }
        op_cmp_gt = { tok_angle_close }
        op_cmp_lt_eq = { tok_angle_open ~ tok_eq }
        op_cmp_gt_eq = { tok_angle_close ~ tok_eq }

        kw_select = { [i"SELECT"] }
        kw_top = { [i"TOP"] }
        kw_from = { [i"FROM"] }
        kw_where = { [i"WHERE"] }
        kw_percent = { [i"PERCENT"] }
        kw_with_ties = { [i"WITH TIES"] }

        kw_or = { [i"OR"] }
        kw_and = { [i"AND"] }
        kw_not = { [i"NOT"] }

        pred_cmp = { expr ~ op_cmp ~ expr }

        clause_where = { kw_where ~ pred_cmp }

        column_name_list = { tok_star }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }

    process! {
        parse_stmt_top(&self) -> ast::Node<ast::TopStatement> {
            (pos: stmt_top
            ,_: kw_top
            ,_: tok_paren_open
            ,expr: parse_expression()
            ,_: tok_paren_close) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::TopStatement {
                    expr: expr,
                    is_legacy: false,
                }
            },
            (pos: stmt_top_legacy
            ,_: kw_top
            ,expr: parse_expression()) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::TopStatement {
                    expr: expr,
                    is_legacy: true,
                }
            },
        } // parse_stmt_top

        parse_expression(&self) -> ast::Node<ast::Expression> {
            (v: lit_integer) => {
                let lit_str = self.input().slice(v.start, v.end);

                ast::Node {
                    pos: ast::Position::from(self.input().line_col(v.start)),
                    value: ast::Expression::Literal {
                        lit: ast::Literal::Int(lit_str.parse().unwrap()),
                    }
                }
            },
            (pos: expr, lit: parse_literal()) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::Expression::Literal {
                    lit: lit
                }
            }
        }

        parse_literal(&self) -> ast::Literal {
            (&v: lit_integer) => ast::Literal::Int(v.parse().unwrap()),
            (&v: lit_bool) => ast::Literal::Bool(v.parse().unwrap()),
        }

        parse_stmt_select(&self) -> ast::Node<ast::SelectStatement> {
            (pos: stmt_select
            ,_: kw_select
            ,stmt_top: parse_stmt_top()
            ) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::SelectStatement {
                    top_statement: Some(stmt_top),
                    column_name_list: vec![],
                }
            },
            (pos: stmt_select
            ,_: kw_select
            ) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::SelectStatement {
                    top_statement: None,
                    column_name_list: vec![],
                }
            },
        } // parse_stmt_select
    }
}

fn main() {
    // TODO: Obviously you never want to compare constants.
    //       So pred_cmp needs to take in idents which means expr needs to
    //       grow to accept more than lit_integer.
    let mut parser = Rdp::new(StringInput::new("SELECT TOP 10 * FROM MyTable"));

    // TODO: This needs to be possible.
    //       `(SELECT TOP 1 Id FROM MyOtherTable)` needs to become a lhs expr
    // let mut parser = Rdp::new(StringInput::new("SELECT * FROM MyTable WHERE (SELECT TOP 1 Id FROM MyOtherTable) < 74"));

    if parser.tsql() {
        println!("{:#?}", parser.queue());
        let stmt = parser.parse_stmt_select();
        println!("{:#?}", stmt);
    } else {
        println!("Failed to parse tsql!");
        println!("Expected: {:?}", parser.expected());
    }
}

mod tests {
    use super::*;

    #[test]
    fn select_top_10_star_from_mytable() {
        let mut parser = Rdp::new(StringInput::new("SELECT TOP (10) * FROM MyTable"));
        assert!(parser.tsql());

        let select = parser.parse_stmt_select();
        assert_eq!(select.pos.to_pair(), (1, 1));

        let select_value = select.value;
        assert!(select_value.top_statement.is_some());

        let top = select_value.top_statement.unwrap();
        assert_eq!(top.pos.to_pair(), (1, 8));

        let top_value = top.value;
        assert!(!top_value.is_legacy);

        let top_expr_value = top_value.expr.value;
        assert_eq!(top_expr_value,
                   ast::Expression::Literal { lit: ast::Literal::Int(10) });
    }

    #[test]
    fn select_top_legacy_10_star_from_mytable() {
        let mut parser = Rdp::new(StringInput::new("SELECT TOP 10 * FROM MyTable"));
        assert!(parser.tsql());

        let stmt_select = parser.parse_stmt_select().value;
        assert!(stmt_select.top_statement.is_some());
    }
}
