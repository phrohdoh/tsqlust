// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#![recursion_limit = "100"]
#![feature(proc_macro)]

#[macro_use]
extern crate pest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

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
            ~ column_name_list ~ kw_from ~ identifier
            ~ clause_where?
        }

        stmt_create_table = {
            kw_create_table ~ identifier
        }

        top_level_repl = _{
            stmt_select
            | stmt_create_table
            | stmt_top_legacy
            | stmt_top
            | expr
            | literal
            | identifier
        }

        expr = {
            lit_integer
        //  | expr_add
        //  | expr_subt
        //  | expr_mult
        //  | expr_div
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

        identifier = @{
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
        kw_with_ties = { [i"WITH"] ~ [i"TIES"] }
        kw_create_table = { [i"CREATE"] ~ [i"TABLE"] }

        kw_or = { [i"OR"] }
        kw_and = { [i"AND"] }
        kw_not = { [i"NOT"] }

        pred_cmp = { expr ~ op_cmp ~ expr }

        clause_where = { kw_where ~ pred_cmp }

        column_name_list = {
            (tok_star | identifier)
            ~ (tok_comma ~ (tok_star | identifier))*
        }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }

    process! {
        parse_stmt_top(&self) -> Option<ast::Node<ast::TopStatement>> {
            (pos: stmt_top
            ,kw: kw_top
            ,paren_open: tok_paren_open
            ,expr: parse_expression()
            ,paren_close: tok_paren_close) => {
                let input = self.input();
                Some(ast::Node {
                    pos: ast::Position::from(input.line_col(pos.start)),
                    tnode: ast::TopStatement {
                        top_keyword: ast::Node {
                            pos: ast::Position::from(input.line_col(kw.start)),
                            tnode: ast::Keyword::Top,
                        },
                        expr: expr,
                        paren_open: Some(ast::Node {
                            pos: ast::Position::from(input.line_col(paren_open.start)),
                            tnode: ast::Token::ParenOpen,
                        }),
                        paren_close: Some(ast::Node {
                            pos: ast::Position::from(input.line_col(paren_close.start)),
                            tnode: ast::Token::ParenClose,
                        }),
                    }
                })
            },

            (pos: stmt_top_legacy
            ,kw: kw_top
            ,expr: parse_expression()) => {
                let input = self.input();
                Some(ast::Node {
                    pos: ast::Position::from(input.line_col(pos.start)),
                    tnode: ast::TopStatement {
                        top_keyword: ast::Node {
                            pos: ast::Position::from(input.line_col(kw.start)),
                            tnode: ast::Keyword::Top,
                        },
                        expr: expr,
                        paren_open: None,
                        paren_close: None,
                    }
                })
            },

            () => None,
        }

        parse_stmt_create_table(&self) -> ast::Node<ast::CreateTableStatement> {
            (_: stmt_create_table
            ,kw: kw_create_table
            ,ident: parse_identifier()) => ast::Node {
                pos: ast::Position::from(self.input().line_col(kw.start)),
                tnode: ast::CreateTableStatement {
                    table_identifier: ident,
                },
            }
        }

        parse_identifier(&self) -> ast::Node<ast::Identifier> {
            (ident: identifier) => {
                let input = self.input();
                ast::Node {
                    pos: ast::Position::from(input.line_col(ident.start)),
                    tnode: ast::Identifier {
                        value: input.slice(ident.start, ident.end).into(),
                    }
                }
            }
        }

        parse_column_name_list(&self) -> ast::Node<ast::ColumnNameList> {
            (pos: column_name_list
            ,cnl_node: parse_column_name_list()) => {
                ast::Node {
                    pos: ast::Position::from(self.input().line_col(pos.start)),
                    tnode: cnl_node.tnode,
                }
            },

            (star: tok_star
            ,_: tok_comma
            ,cnl_node: parse_column_name_list()) => {
                let pos = ast::Position::from(self.input().line_col(star.start));
                let mut nodes = cnl_node.tnode.identifiers;
                nodes.insert(0,
                    ast::Node {
                        pos: pos,
                        tnode: ast::Identifier {
                            value: "*".into(),
                        }
                    }
                );

                ast::Node {
                    pos: pos,
                    tnode: ast::ColumnNameList {
                        identifiers: nodes,
                    }
                }
            },

            (star: tok_star) => {
                let pos = ast::Position::from(self.input().line_col(star.start));
                ast::Node {
                    pos: pos,
                    tnode: ast::ColumnNameList {
                        identifiers: vec![
                            ast::Node {
                                pos: pos,
                                tnode: ast::Identifier {
                                    value: "*".into(),
                                }
                            }
                        ]
                    }
                }
            },

            (ident_node: parse_identifier()
            ,_: tok_comma
            ,cnl_node: parse_column_name_list()) => {
                let pos = ident_node.pos;
                let mut nodes = cnl_node.tnode.identifiers;
                nodes.insert(0, ident_node);

                ast::Node {
                    pos: pos,
                    tnode: ast::ColumnNameList {
                        identifiers: nodes,
                    }
                }
            },

            (ident_node: parse_identifier()) => {
                ast::Node {
                    pos: ident_node.pos,
                    tnode: ast::ColumnNameList {
                        identifiers: vec![
                            ast::Node {
                                pos: ident_node.pos,
                                tnode: ident_node.tnode
                            }
                        ],
                    }
                }
            },
        }

        parse_expression(&self) -> ast::Node<ast::Expression> {
            (v: lit_integer) => {
                let input = self.input();
                ast::Node {
                    pos: ast::Position::from(input.line_col(v.start)),
                    tnode: ast::Expression::Literal {
                        lit: ast::Literal::Int(input.slice(v.start, v.end).parse().unwrap()),
                    }
                }
            },
            (pos: expr, lit: parse_literal()) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                tnode: ast::Expression::Literal {
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
            ,_: kw_from
            ,table_ident: parse_identifier()
            ) => ast::Node {
                pos: ast::Position::from(self.input().line_col(pos.start)),
                tnode: ast::SelectStatement {
                    top_statement: stmt_top,
                    column_name_list: columns,
                    table_identifier: table_ident,
                }
            },
        }
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

    let select_node_node = select_node.tnode;

    if let Some(top_node) = select_node_node.top_statement {
        vis.visit_top_statement(&mut ctx, &top_node);
    }

    let columns_node = select_node_node.column_name_list;
    vis.visit_column_name_list(&mut ctx, &columns_node);

    Ok(ctx.get_diagnostics())
}

/// This is a temporary function used by the WIP graphical interface.
/// You should not rely on it or expect it to exist in any following versions.
pub fn parse_tsql_select(tsql: &str) -> Result<ast::Node<ast::SelectStatement>, String> {
    let mut parser = Rdp::new(StringInput::new(tsql));

    if !parser.stmt_select() {
        return Err("Failed to parse SELECT statement.".into());
    }

    Ok(parser.parse_stmt_select())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Rdp, StringInput, ast};

    #[allow(unused_imports)]
    use pest::Parser;

    use ast::*;

    #[test]
    fn column_name_list_star() {
        let mut parser = Rdp::new(StringInput::new("*"));
        assert!(parser.column_name_list());

        let found = parser.parse_column_name_list().tnode.identifiers;

        let expected = vec![
            Node {
                pos: Position { line: 1, col: 1 },
                tnode: Identifier { value: "*".into() },
            }
        ];

        assert_eq!(found, expected);
    }

    #[test]
    fn column_name_list_identifiers() {
        let mut parser = Rdp::new(StringInput::new(r#"Id,SomeColumn
        ,ColumnA,  ColumnB
        ,  Foo  ,Qux"#));
        assert!(parser.column_name_list());

        let found = parser.parse_column_name_list().tnode.identifiers;

        let expected = vec![
            Node {
                pos: Position { line: 1, col: 1 },
                tnode: Identifier { value: "Id".into() },
            },
            Node {
                pos: Position { line: 1, col: 4 },
                tnode: Identifier { value: "SomeColumn".into() },
            },
            Node {
                pos: Position { line: 2, col: 10 },
                tnode: Identifier { value: "ColumnA".into() },
            },
            Node {
                pos: Position { line: 2, col: 20 },
                tnode: Identifier { value: "ColumnB".into() },
            },
            Node {
                pos: Position { line: 3, col: 12 },
                tnode: Identifier { value: "Foo".into() },
            },
            Node {
                pos: Position { line: 3, col: 18 },
                tnode: Identifier { value: "Qux".into() },
            }
        ];

        assert_eq!(expected, found);
    }

    #[test]
    fn select_top_10_star_from_mytable() {
        let mut parser = Rdp::new(StringInput::new("SELECT TOP (10) * FROM MyTable"));
        assert!(parser.tsql());

        let stmt_select = parser.parse_stmt_select();
        assert_eq!(stmt_select.pos.to_pair(), (1, 1));

        let select_node = stmt_select.tnode;
        let top = select_node.top_statement.unwrap();
        assert_eq!(top.pos.to_pair(), (1, 8));

        let top_node = top.tnode;
        assert!(!top_node.is_legacy());

        let top_expr_node = top_node.expr.tnode;
        assert_eq!(top_expr_node,
                   Expression::Literal { lit: Literal::Int(10) });

        let column_idents = select_node.column_name_list.tnode.identifiers;
        assert_eq!(column_idents, vec![
            Node {
                pos: Position {
                    line: 1,
                    col: 17,
                },
                tnode: Identifier {
                    value: "*".into(),
                }
            }
        ]);
    }

    #[test]
    fn select_top_legacy_10_star_from_mytable() {
        let mut parser = Rdp::new(StringInput::new("SELECT TOP 10 * FROM MyTable"));
        assert!(parser.tsql());

        let stmt_select = parser.parse_stmt_select().tnode;
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

        let stmt_top = parser.parse_stmt_top().unwrap().tnode;
        assert!(!stmt_top.is_legacy());

        assert_eq!(stmt_top.expr.tnode,
                   Expression::Literal { lit: Literal::Int(972) });
    }
}
