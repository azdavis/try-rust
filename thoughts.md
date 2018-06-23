# thoughts

## ownership

- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## references

- At any given time, you can have either one mutable reference or any number of
  immutable references.
- References must always be valid.
