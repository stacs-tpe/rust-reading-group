
# To Panic or Not to Panic?

Use `panic!` when:

- You are in a **bad state**.
- Code **relies** on correct assumptions.
- Error is **unexpected**.
- Cannot easily encode the requirement in types.

Otherwise: propagate errors with `Result`.

