# `tsqlust`

### A T-SQL lexer/parser/static-analysis framework

[Documentation](https://phrohdoh.github.io/tsqlust/tsqlust/index.html)

## Project Goals

> Provide developers with the ability to statically verify and analyze T-SQL queries

How this is done:

1. Create an AST from input (typically from a file)
2. You will implement the provided `Visitor` trait
3. Invoke `tsqlust::get_diagnostics_for_query(&your_sql)`
4. Iterate over the resulting `Vec<tsqlust::diagnostics::Diagnostic>` and take action

## Usage Examples

See `src/bin/main.rs`

## LICENSE

GPLv3

## TODO

All of the GitHub Issues are todo and are up for grabs!

* Write more tests

## Other

Please review `CLA.md` before making contributions.

This CLA is similar to those that Google, the Apache Foundation, Dropbox,

and many others require contributors to sign before accepting contributions.

The purpose of the CLA is to ensure that the project author may use the

resulting works in whatever way they believe most benefits the project.