# thoughts

## ownership

- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## references

- At any given time, you can have either one mutable reference or any number of
  immutable references.
- References must always be valid.

## lifetimes

- Every reference has a lifetime.
- The main aim of lifetimes is to prevent dangling references.
- Most of the time, lifetimes are implicit and inferred, just like most of the
  time, types are inferred.
- We must annotate types when multiple types are possible.
- In a similar way, we must annotate lifetimes when the lifetimes of references
  could be related in a few different ways.
- Lifetime annotations don't change how long any of the references live.
- The lifetime elision rules:
  1. Each parameter that is a reference gets its own lifetime parameter.
  2. If there is exactly one input lifetime parameter, that lifetime is assigned
     to all output lifetime parameters.
  3. If there are multiple input lifetime parameters, but one of them is &self
     or &mut self because this is a method, the lifetime of self is assigned to
     all output lifetime parameters.
