# SE-Rust
SEstudio - Hello Rust

## Resource
* Home page: https://www.rust-lang.org
* Document: https://www.rust-lang.org/learn

# RUST BASIC Syntax

## 1 - First app "Hello word"

This is the source code of the traditional Hello world program

Create two directories with: 
* **src** to store source code Rust.
* **bin** to store binary file after Rust compile.

```rust
// ./src/hello.rs
fn main() {
  println!("Hello world!!!");
}
```

**println!** is a macro that prints text *"Hello world!!!"* to the console.

A binary can be generated using the Rust compiler: **rustc**

```shell
$ rustc ./src/hello.rs -o ./bin/hello
```

**rustc** will produce a hello binary that can be executed. 

```shell
$ ./bin/hello
Hello world!!!
```

## 2 - Variables and Mutability

### Variables - immutable
Variables only store variable with keyword ***mut*** when define variable.

```rust
let x = 5;
println!("The value of x is: {x}");

// mutable variable
let mut y = 6;  // define variable - can change value
println!("The value of y is: {y}");

y = 100; 
println!("The value of y is: {y}");
```

### Constants 
Constants will be define with keyword ***const***

```rust
// constants - const
const PI : f32 = 3.14;    // float with 32 bit
println!("PI = {PI}");
```

### Shadowing
The value of variable will be store with keyword ***let***. But we can change the value when reuse keyword ***let*** again, but it will be create new memory and have scope inside code block

```rust
// Shadowing -> update value of immutable variable
let z = 100; 
println!("The value of z is: {z}");

let z = 99;
println!("The value of z is: {z}"); 

{
  // variable z have new memory inside this code block. So it difficult with z outside block
  let z = 888;
  println!("The value of z is: {z}"); 
}

println!("The value of z is: {z}"); 
```

Variable will not be change data-type if it is defined with ***mut*** keyword

```rust
// variable cannot change data-type if we using mut keyword
let value = "abc def"; 
let value = value.len(); 
println!("Value is: {value}");

// The code block will be error because change data-type of mutable variable
let mut mut_value = "abc def";  // &str
// value = mut_value.len();        // usize -> error
println!("Value is: {value}");
```

