# commentremover_rust
A simple program that removes comments from files.

## Examples

Input
```rust
use std::fs;

//This comment will be removed
fn main() {
    //This comment will also be removed
    println!("Hello, World!");//And this!
}
```
Output
```rust
use std::fs;

fn main() {
    println!("Hello, World!");
}
```

