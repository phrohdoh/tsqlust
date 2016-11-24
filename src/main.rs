#![recursion_limit = "200"]

#[macro_use]
extern crate pest;

use pest::prelude::*;

impl_rdp! {
    grammar! {
        tsql = { (
            stmt_select
        ) ~ eoi }

        stmt_top_legacy = { kw_top ~ lit_integer ~ kw_percent? }
        stmt_top = { kw_top ~ tok_paren_open ~ lit_integer ~ tok_paren_close ~ kw_percent? ~ kw_with_ties? }

        stmt_select = {
            kw_select
            ~ (stmt_top | stmt_top_legacy)?
            ~ column_name_list ~ kw_from ~ term_id
            ~ clause_where?
        }

        expr = {
            lit_integer |
            expr_add |
            expr_subt |
            expr_mult |
            expr_div
        }

        expr_add = { expr ~ tok_plus ~ expr }
        expr_subt = { expr ~ tok_minus ~ expr }
        expr_mult = { expr ~ tok_star ~ expr }
        expr_div = { expr ~ tok_slash_forward ~ expr }

        lit_integer = @{ ['0'..'9']+ }

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
        // tok_eq_eq = { tok_eq ~ tok_eq } // TODO: This is valid somewhere, where?
        tok_bang = @{ ["!"] }
        tok_angle_open = @{ ["<"] }
        tok_angle_close = @{ [">"] }

        op_cmp = {
            op_cmp_eq |
            // op_cmp_eq_eq |
            op_cmp_neq_bang |
            op_cmp_lt |
            op_cmp_gt |
            op_cmp_lt_eq |
            op_cmp_gt_eq
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

        // TODO: column_name_list
        column_name_list = { tok_star }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }
}

fn main() {
    // TODO: Obviously you never want to compare constants.
    //       So pred_cmp needs to take in idents which means expr needs to
    //       grow to accept more than lit_integer.
    let mut parser = Rdp::new(StringInput::new("SELECT * FROM MyTable WHERE 4 < 5"));

    // TODO: This needs to be possible.
    //       `(SELECT TOP 1 Id FROM MyOtherTable)` needs to become a lhs expr
    //let mut parser = Rdp::new(StringInput::new("SELECT * FROM MyTable WHERE (SELECT TOP 1 Id FROM MyOtherTable) < 74"));

    if parser.tsql() {
        println!("{:#?}", parser.queue());
    } else {
        println!("Failed to parse tsql!");
        println!("Expected: {:?}", parser.expected());
    }
}
