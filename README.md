# `tsqlust`

### A T-SQL lexer/parser/static-analysis framework

[Documentation](https://phrohdoh.github.io/tsqlust/tsqlust/index.html)

## Project Goals

> Provide developers with the ability to statically verify and analyze T-SQL queries

How this is done:

1. Create an AST from input (typically from a file)
2. You will implement the provided `Visitor` trait
3. Invoke `tsqlust::get_diagnostics_for_tsql(&your_sql)`
4. Iterate over the resulting `Vec<tsqlust::diagnostics::Diagnostic>` and take action

## Usage

See the `tests/` and `examples/` directories.

Example REPL session:
```
Enter 'q' at any time to quit.
Enter '?' at any time for help.

>> SELECT TOP 15 * FROM MyTable
Node {
    pos: Position {
        line: 1,
        col: 1
    },
    value: SelectStatement {
        top_statement: Some(
            Node {
                pos: Position {
                    line: 1,
                    col: 8
                },
                value: TopStatement {
                    top_keyword_pos: Position {
                        line: 1,
                        col: 8
                    },
                    expr: Node {
                        pos: Position {
                            line: 1,
                            col: 12
                        },
                        value: Literal {
                            lit: Int(
                                15
                            )
                        }
                    },
                    paren_open: None,
                    paren_close: None
                }
            }
        ),
        column_name_list: Node {
            pos: Position {
                line: 1,
                col: 15
            },
            value: ColumnNameList {
                column_names: [
                    "*"
                ]
            }
        }
    }
}
>> q
Goodbye!
```

## LICENSE

GPLv3

## TODO

All of the GitHub Issues are todo and are up for grabs!

* Write more tests

## Supporting this project

If you would like to financially support this project please do the following:
* [Become a patron](https://www.patreon.com/Phrohdoh)
* [Tip me on gratipay](https://gratipay.com/~Phrohdoh/)
* [E-mail me](mailto:taryn@phrohdoh.com) for one-time donation information