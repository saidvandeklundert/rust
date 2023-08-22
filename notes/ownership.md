

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.


`Copy`: variables that use it do not move, but are copied