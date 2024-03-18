**flat-parser**
- [x] Refactor ./src/lib.rs
- [x] Add ./src/utils.rs
- [ ] Refactor ./src/parser.rs
- [ ] Implement ./src/investigate.rs
- [x] Support for line breaks (\n or \r\n)
- [ ] Supports line breaks in interactive mode (\)
- [x] Add unit test, ./src/lexer.rs
- [ ] Add unit test, ./src/parser.rs
- [x] Add unit test, ./src/utils.rs
- [ ] Remaking the Parser ./src/parser.rs
- [ ] Add Conditional statements: if, else and loop statement.
- [ ] Support Close FD
- [ ] Support Shell Redirect (no command redirect)


**flat-builtin**
- [x] Implement ./src/calc.rs
- [ ] Handle negative values, ./src/calc.rs
- [ ] Correct Error::DUMMY to the appropriate Error and ErrorKind. ./src/calc.rs
- [ ] Reimplement using flat-parser. Currently, it is self-contained in the code for this context. ./src/calc.rs