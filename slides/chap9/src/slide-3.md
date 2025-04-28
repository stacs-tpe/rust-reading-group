
# Error Handling Philosophy

- Rust forces you to **acknowledge** the possibility of errors.
- Program must handle errors **before** it will compile.
- Two kinds of errors:
  - **Unrecoverable** → use `panic!`
  - **Recoverable** → use `Result<T, E>`

