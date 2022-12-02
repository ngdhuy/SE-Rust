# SE-Rust
SEstudio - Hello Rust

## Resource
* Home page: https://www.rust-lang.org
* Document: https://www.rust-lang.org/learn

# RUST BASIC Syntax

## First app - Hello word

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