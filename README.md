# Circuit
A compiled / interpreted language made in Rust.

## Installation

Add this to your `Cargo.toml`
```TOML
[dependencies]
circuit-lang = "0.1.0"
```
 
## Example
```rust
extern crate circuit_lang as circuit;

use circuit::lexer;
use circuit::parser::{ParseStream, parse};

fn main() {
   let tokens = lexer::tokenize("1+2").expect("Failed to tokenize input!");
   let mut stream = ParseStream::new(tokens);

   // Parses an add expression or one with lower precedence.
   println!("Generated AST: {:#?}", parse::add(&mut stream));
}
```

## Finished syntax

```
use std::env;

fun main() {
   printf("The name of this program is {0}", env::arg(0));
   print("Hello, World!");
}
```


