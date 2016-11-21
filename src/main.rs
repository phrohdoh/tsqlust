#[macro_use]
extern crate pest;

use pest::prelude::*;

impl_rdp! {
    grammar! {
        tsql = { (
            stmt_select
        ) ~ eoi }

        stmt_top_legacy = { kw_top ~ lit_integer ~ kw_percent? }
        stmt_top = { kw_top ~ tok_paren_open ~ expr ~ tok_paren_close ~  kw_percent? ~ kw_with_ties? }

        stmt_select = { kw_select ~
            (tok_star | (stmt_top | stmt_top_legacy)) ~
            kw_from ~ term_id ~
            clause_where?
        }

        // TODO: expr
        expr = { lit_integer }

        lit_integer = @{ ['0'..'9']+ }

        term_id = @{ (
            ['a'..'z'] |
            ['A'..'Z'] |
            ['0'..'9'] |
            ["_"]
        )+ }

        tok_star = { ["*"] }
        tok_eq = { ["="] }
        // tok_eq_eq = { tok_eq ~ tok_eq } // TODO: This is valid somewhere, where?
        tok_paren_open = { ["("] }
        tok_paren_close = { [")"] }

        kw_select = { [i"SELECT"] }
        kw_top = { [i"TOP"] }
        kw_from = { [i"FROM"] }
        kw_where = { [i"WHERE"] }
        kw_percent = { [i"PERCENT"] }
        kw_with_ties = { [i"WITH TIES"] }

        kw_or = { [i"OR"] }
        kw_and = { [i"AND"] }
        kw_not = { [i"NOT"] }

        pred_cmp = { expr ~ tok_eq ~ expr }

        clause_where = { kw_where ~ pred_cmp }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }
}

fn main() {
    // TODO: Obviously you never want to compare constants.
    //       So pred_cmp needs to take in idents which means expr needs to
    //       grow to accept more than lit_integer.
    let mut parser = Rdp::new(StringInput::new("SELECT * FROM MyTable WHERE 5 = 5"));

    if parser.tsql() {
        println!("{:#?}", parser.queue());
    } else {
        println!("Failed to parse tsql!");
        println!("Expected: {:?}", parser.expected());
    }
}
