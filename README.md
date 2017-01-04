# `tsqlust`

### A T-SQL lexer/parser/static-analysis framework

[Documentation](https://phrohdoh.github.io/tsqlust/tsqlust/index.html)

## Project Goals

> Provide human-friendly errors, warnings, and messages when writing tsql queries

How this is done:

1. Create an AST from input (typically from a file)
2. You will implement the provided [`Visitor`](https://phrohdoh.github.io/tsqlust/tsqlust/visitor/trait.Visitor.html) trait
3. Invoke `tsqlust::get_diagnostics_for_tsql(&your_sql_str)`
4. Iterate over the resulting `Vec<tsqlust::diagnostics::Diagnostic>` and take action

## Project Status

> Currently the project only lexes and parses a basic query

## Usage

See the `tests/` and `examples/` directories.

Example REPL session (start via `cargo run`):
```
>> SELECT TOP (5) Id,FirstName,   LastName   ,* FROM MyTable
Node {
    pos: Position {
        line: 1,
        col: 1
    },
    tnode: SelectStatement {
        top_statement: Some(
            Node {
                pos: Position {
                    line: 1,
                    col: 8
                },
                tnode: TopStatement {
                    top_keyword: Node {
                        pos: Position {
                            line: 1,
                            col: 8
                        },
                        tnode: Top
                    },
                    expr: Node {
                        pos: Position {
                            line: 1,
                            col: 13
                        },
                        tnode: Literal {
                            lit: Int(
                                5
                            )
                        }
                    },
                    paren_open: Some(
                        Node {
                            pos: Position {
                                line: 1,
                                col: 12
                            },
                            tnode: ParenOpen
                        }
                    ),
                    paren_close: Some(
                        Node {
                            pos: Position {
                                line: 1,
                                col: 14
                            },
                            tnode: ParenClose
                        }
                    )
                }
            }
        ),
        column_name_list: Node {
            pos: Position {
                line: 1,
                col: 16
            },
            tnode: ColumnNameList {
                identifiers: [
                    Node {
                        pos: Position {
                            line: 1,
                            col: 16
                        },
                        tnode: Identifier {
                            value: "Id"
                        }
                    },
                    Node {
                        pos: Position {
                            line: 1,
                            col: 19
                        },
                        tnode: Identifier {
                            value: "FirstName"
                        }
                    },
                    Node {
                        pos: Position {
                            line: 1,
                            col: 32
                        },
                        tnode: Identifier {
                            value: "LastName"
                        }
                    },
                    Node {
                        pos: Position {
                            line: 1,
                            col: 44
                        },
                        tnode: Identifier {
                            value: "*"
                        }
                    }
                ]
            }
        },
        table_identifier: Node {
            pos: Position {
                line: 1,
                col: 51
            },
            tnode: Identifier {
                value: "MyTable"
            }
        }
    }
}
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
