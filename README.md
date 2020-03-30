# curry-macro

Have fun currying using Rust's native closure syntax.

# Example

You can curry an add function that adds two numbers.

First, import the curry macro:
```rust
use curry_macro::curry;
```
Then, you have three forms of function definitions to choose from:

1. Simplest form, without any type annotations.
You need to use the curried function so that
the rust compiler can infer the input and return
types for you:
```rust
let add = curry!(|a, b| a + b);
assert_eq!(add(1)(2), 3);
```

2. With input type annotations:
```rust
let add = curry!(|a: i32, b: i32| a + b);
```

3. With input and return type annotations and a block as function body
```rust
let add = curry!(|a: i32, b: i32| -> i32 { a + b });
```

The above three functions work the same:
```rust
// You can generate intermediate functions that are partially applied:
let add1_to = add(1);
let sum = add1_to(2);
assert_eq!(sum, 3);
// You can also can apply all arguments at once:
let sum = add(1)(2);
assert_eq!(sum, 3);
```

# Credits
Big thanks to Korede-TA for his awesome [curry-macro](https://github.com/Korede-TA/curry-macro).

# License
MIT