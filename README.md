# SE-Rust
SEstudio - Hello Rust

## Resource
* Home page: https://www.rust-lang.org
* Document: https://www.rust-lang.org/learn

# RUST BASIC Syntax

## 1 - First app "Hello word"

This is the source code of the traditional Hello world program
* Source: lab01-basic/src/demo01.rs

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

* Source: lab01-basic/src/demo02.rs

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

## 3 - Data types

Rust look at two data types subsets: scalar and compound. 

* Source: lab01-basic/src/demo03.rs

### Scalar types

A scalar type represents a single value. Rust has four primary scalar types: ***integers, floating-point numbers, Booleans and characters***. 

### a. Integer types

**Integer types in Rust**

| Length | Signed    | Unsigned |
| :--- | :--- | :--- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

**Integer Literals in Rust**

| Number literals | Example |
| :--- | :-- |
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

### b. Floating-Point types

Rust has two primitive types for **floating-point numbers**. They are ***f32*** and ***f64***

```rust
fn main() {
  let x = 2.0;       // f64

  let y: f32 = 3.0;  // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. 

* The **f32** type is a single-precision ***float***. 
* The **f64** type has ***double*** precision.

### c. Numeric Operations

Rust supports the basic mathematical operations you'd expect for all of the number types: addition, subtraction, multiplication, division and remainder. Integer division rounds down to the nearest integer. The following code shows how you'd use each numeric operation in a let statement:

```rust
fn main() {
  // addition
  let sum = 5 + 10; 

  // subtraction
  let difference = 95.5 - 4.3; 

  // multiplication
  let product = 4 * 30; 

  // remainder
  let remainder = 43 % 5;
}
```
### d. The Boolean types

Boolean type in Rust has two possible values: ***true*** and ***false***. It has ***one*** byte in size. Rust use keyword ***bool*** for code.

```rust
fn main() {
  let t = true; 

  let f: bool = false; // with explicit type annotation
}
```

### e. The Character types

Rust's char type is the languages's most primitive alphabetic type.

```rust
fn main() {
  let c = 'z';
  let z: char = 'Z';
  let heart_eyed_cat = '🙀';
}
```

Notes: 
* That we specify char literals with ***single quotes***, as opposed to string literals, which use ***double quotes***. Rust's char type is ***four bytes*** in size and represents a Unicode Scalar value, which means it can represent a lot more than just ASCII. 
* Accented letters: Chinese, Japanese and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
* Unicode Scalar Values range from ***U+0000*** to ***U+D7FF*** and ***U+E000*** to ***U+10FFFF*** inclusive.
* However, a "character" isn't really a concept in Unicode, so your human intuition for what a "character" is may not match up with what a char is in Rust.

### f. Compound types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### The Tuple type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.

Tuple have a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (k, m, l) = tup;

  println!("k is {k} with data-type is {}", type_of(k));
  println!("m is {m} with data-type is {}", type_of(m));
  println!("l is {l} with data-type is {}", type_of(l));

  // access element in tuple with index
  println!("First item of tuple at index(0) {}", tup.0);
  println!("Second item of tuple at index(1) {}", tup.1);
  println!("Third item of tuple at index(2) {}", tup.2);
}
```
#### The Array type

Another way to have a collection of multiple values is with an array.
Unlike a tuple, every element of an array must have the same type.

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  let num = ["one", "two", "three", "four", "five"];
}
```

* Accessing array elements

```rust
fn main() {
  // define data-type and number of item for array 
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  // access item/element with index
  let first = arr[0]; 
  let second = arr[1];
}
```

* Invalid array elements access

Let's see what happens if we try to access an element of an array that is past the end of the array.