// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#![recursion_limit = "100"]

#[macro_use]
extern crate pest;

use pest::{StringInput, Parser};
use pest::prelude::{Token, Input};

pub mod ast;
pub mod visitor;
pub mod diagnostics;

impl_rdp! {
    grammar! {
        tsql = _{ (
            stmt_select
        ) ~ eoi }

// _anything = { any }

        stmt_top_legacy = { kw_top ~ lit_integer ~ kw_percent? }
        stmt_top = { kw_top ~ tok_paren_open ~ expr ~ tok_paren_close ~ kw_percent? ~ kw_with_ties? }

        stmt_select = {
            kw_select
            ~ (stmt_top | stmt_top_legacy)?
            ~ column_name_list ~ kw_from ~ term_id
            ~ clause_where?
        }

        top_level_repl = _{
            stmt_select
            | stmt_top_legacy
            | stmt_top
            | expr
            | literal
        }

        expr = {
            lit_integer
// | expr_add
// | expr_subt
// | expr_mult
// | expr_div
        }

// expr_add = { expr ~ tok_plus ~ expr }
// expr_subt = { expr ~ tok_minus ~ expr }
// expr_mult = { expr ~ tok_star ~ expr }
// expr_div = { expr ~ tok_slash_forward ~ expr }

        lit_bool = { [i"TRUE"] | [i"FALSE"] }
        lit_integer = @{ ['0'..'9']+ }

        literal = {
            lit_bool
            | lit_integer
        }

        term_id = @{
            (['a'..'z'] | ['A'..'Z'] | ["_"])
            ~ (['a'..'z'] | ['A'..'Z'] | ['0'..'9'] | ["_"])*
        }

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

        tok_comma = { [","] }

        tok_eq = { ["="] }
//  tok_eq_eq = @{ tok_eq ~ tok_eq } // TODO: This is valid somewhere, where?
        tok_bang = { ["!"] }
        tok_angle_open = { ["<"] }
        tok_angle_close = { [">"] }

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
//  op_cmp_eq_eq = @{ tok_eq ~ tok_eq }
        op_cmp_neq_bang = @{ tok_bang ~ tok_eq }
        op_cmp_lt = { tok_angle_open }
        op_cmp_gt = { tok_angle_close }
        op_cmp_lt_eq = @{ tok_angle_open ~ tok_eq }
        op_cmp_gt_eq = @{ tok_angle_close ~ tok_eq }

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

        column_name_list = {
            (tok_star | term_id)
            ~ (tok_comma ~ (term_id | tok_star))*
        }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }

    process! {
        parse_stmt_top(&self) -> Option<ast::Node<ast::TopStatement>> {
            (pos: stmt_top
            ,kw: kw_top
            ,paren_open: tok_paren_open
            ,expr: parse_expression()
            ,paren_close: tok_paren_close) => Some(ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::TopStatement {
                    top_keyword_pos: ast::Position::from(self.input().line_col(kw.start)),
                    expr: expr,
                    paren_open: Some(ast::Node {
                        pos: ast::Position::from(self.input().line_col(paren_open.start)),
                        value: ast::Token::ParenOpen,
                    }),
                    paren_close: Some(ast::Node {
                        pos: ast::Position::from(self.input().line_col(paren_close.start)),
                        value: ast::Token::ParenClose,
                    }),
                }
            }),
            (pos: stmt_top_legacy
            ,kw: kw_top
            ,expr: parse_expression()) => Some(ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::TopStatement {
                    top_keyword_pos: ast::Position::from(self.input().line_col(kw.start)),
                    expr: expr,
                    paren_open: None,
                    paren_close: None,
                }
            }),
            () => None,
        } // parse_stmt_top

        parse_column_name_list(&self) -> ast::Node<ast::ColumnNameList> {
            (columns: column_name_list) => {
                let pos = self.input().line_col(columns.start);
                let col_str = self.input().slice(columns.start, columns.end);
                let mut names_and_start_pos = Vec::new();

                let mut in_word = false;
                let mut word = String::new();
                let mut start_pos = 0usize;

                let len = col_str.len();
                for (pos, ch) in col_str.char_indices() {
                    if ch.is_whitespace() {
                        continue;
                    }

                    if ch == ',' {
                        in_word = false;
                        names_and_start_pos.push((word, start_pos));
                        start_pos = 0usize;
                        word = String::new();
                        continue;
                    } else if pos + ch.len_utf8() == len {
                        in_word = false;
                        word.push(ch);
                        names_and_start_pos.push((word, start_pos));
                        word = String::new();
                    } else if in_word {
                        word.push(ch);
                    } else {
                        in_word = true;
                        start_pos = pos;
                        word.push(ch);
                    }
                }

                ast::Node {
                    pos: ast::Position::from(pos),
                    value: ast::ColumnNameList {
                        identifiers: names_and_start_pos.into_iter().map(|(n, p)| ast::Node {
                            pos: ast::Position::from(self.input().line_col(p + columns.start)),
                            value: ast::Identifier {
                                value: n,
                            }
                        }).collect(),
                    }
                }
            }
        }

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
            ,columns: parse_column_name_list()
            ) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                value: ast::SelectStatement {
                    top_statement: stmt_top,
                    column_name_list: columns,
                }
            },
        } // parse_stmt_select
    }
}

/// This is a helper function so that example code can be written.
/// You should not rely on it or expect it to exist in any following versions.
pub fn get_diagnostics_for_tsql(query_string: &str,
                                vis: &mut visitor::Visitor)
                                -> Result<Vec<diagnostics::Diagnostic>, (String, String)> {
    let mut ctx = diagnostics::Context::new();
    let mut parser = Rdp::new(StringInput::new(query_string));

    if !parser.tsql() {
        let q = format!("{:?}", parser.queue());
        let e = format!("{:?}", parser.expected());
        // Ideally we could do something like this (ref https://github.com/dragostis/pest/issues/90):
        // let (expected_toks, idx) = parser.expected();
        // parser._anything();
        // parser.set_pos(idx - 1);
        // let q = parser.queue();
        // => found `q` expected one of `expected_toks`
        return Err((q, e));
    }

    let select_node = parser.parse_stmt_select();
    vis.visit_select_statement(&mut ctx, &select_node);

    if let Some(top_node) = select_node.value.top_statement {
        vis.visit_top_statement(&mut ctx, &top_node);
    }

    Ok(ctx.get_diagnostics())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Rdp, StringInput, ast};

    #[allow(unused_imports)]
    use pest::Parser;

    #[test]
    fn column_name_list_star() {
        let mut parser = Rdp::new(StringInput::new("*"));
        assert!(parser.column_name_list());

        let columns = parser.parse_column_name_list().value.column_names;
        assert_eq!(columns, vec!["*"]);
    }

    #[test]
    fn column_name_list_term_ids() {
        let mut parser = Rdp::new(StringInput::new("Id,SomeColumn,ColumnA,  ColumnB,  Foo  ,Qux"));
        assert!(parser.column_name_list());

        let columns = parser.parse_column_name_list().value.column_names;
        assert_eq!(columns,
                   vec!["Id", "SomeColumn", "ColumnA", "ColumnB", "Foo", "Qux"]);
    }

    #[test]
    fn select_top_10_star_from_mytable() {
        let mut parser = Rdp::new(StringInput::new("SELECT TOP (10) * FROM MyTable"));
        assert!(parser.tsql());

        let stmt_select = parser.parse_stmt_select();
        assert_eq!(stmt_select.pos.to_pair(), (1, 1));

        let select_value = stmt_select.value;
        let top = select_value.top_statement.unwrap();
        assert_eq!(top.pos.to_pair(), (1, 8));

        let top_value = top.value;
        assert!(!top_value.is_legacy());

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

    // TODO: Uncomment once https://github.com/dragostis/pest/issues/84 is fixed.
    // #[test]
    // fn top_fail_to_parse_expect_open_paren() {
    //     let mut parser = Rdp::new(StringInput::new("TOP 509345)"));
    //     assert!(!parser.stmt_top());
    //     let (expected, _) = parser.expected();
    //     assert_eq!(expected, Rule::tok_paren_open);
    // }

    #[test]
    fn top_972() {
        let mut parser = Rdp::new(StringInput::new("TOP (972)"));
        assert!(parser.stmt_top());

        let stmt_top = parser.parse_stmt_top().unwrap().value;
        assert!(!stmt_top.is_legacy());

        assert_eq!(stmt_top.expr.value,
                   ast::Expression::Literal { lit: ast::Literal::Int(972) });
    }
}
