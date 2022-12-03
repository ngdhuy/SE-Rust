# Rust syntax by Example

Rust is a modern system programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

Rust by example (RBE) is a collection of runnable examples that illustrate various Rust concepts and standard libraries. To get even more out of these example, don't forget to install Rust locally and checkout the official docs. Additionally for the curious, you can also checkout the source code for this lab from url [https://doc.rust-lang.org/stable/rust-by-example/index.html](https://doc.rust-lang.org/stable/rust-by-example/index.html). 

Now is the list of examples: 

1. **Hello World** - Start with a traditional Hello World program.

2. **Primitives** - Learn about signed integers, unsigned integers and other primitives.

3. **Custom Types** - struct and enum.

4. **Variable Bindings** - mutable bindings, scope, shadowing.

5. **Types** - Learn about changing and defining types.

6. **Conversion**

7. **Expressions**

8. **Flow of Control** - if/else, for, and others.

9. **Functions** - Learn about Methods, Closures and High Order Functions.

10. **Modules** - Organize code using modules

11. **Crates** - A crate is a compilation unit in Rust. Learn to create a library.

12. **Cargo** - Go through some basic features of the official Rust package management tool.

13. **Attributes** - An attribute is metadata applied to some module, crate or item.

14. **Generics** - Learn about writing a function or data type which can work for multiple types of arguments.

15. **Scoping rules** - Scopes play an important part in ownership, borrowing, and lifetimes.

16. **Traits** - A trait is a collection of methods defined for an unknown type: Self

17. **Macros**

18. **Error handling** - Learn Rust way of handling failures.

19. **Std library types** - Learn about some custom types provided by std library.

20. **Std misc** - More custom types for file handling, threads.

21. **Testing** - All sorts of testing in Rust.

22. **Unsafe Operations**

23. **Compatibility**

24. **Meta** - Documentation, Benchmarking.

# List of Rust By Examples
## **1 - Hello World**

Reference: [./src/demo01.rs](./src/demo01.rs)

### ***1.1 - Comment***

Rust support a few different varieties: 

* Regular comment with are ignored by the compiler:

```rust
// line comments which go to the end of the line.
/* block comments which go to the closing delimiter. */
```

* Doc comments which are parsed into HTML library documentation (at 24. Meta -> 24.1  Document): 

```rust
/// generate library docs for the following item. 
//! generate library docs for the enclosing item.
```

### ***1.2 - Formatted print***

Printing is handled by a series of macro defined std::fmt some of which include:

* ***format!*** - write formatted text to String
* ***print!*** - same as *format!* but the text is printed to the console (io::stdout)
* ***println!*** - same as *print!* but a newline is appended.
* ***eprint!*** - same as *print!* but the text is printed to the standard error (io::stderr). 
* ***eprintln!*** - same as *eprint!* but a newline appended.

```rust
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character after a
    // `:`.
    println!("Base 10:               {}",   69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C


    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    //and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);


    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```
#### ***1.2.1 - Debug***

Using ```{:?}``` to debug

```rust
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

Pretty printing with ***{:#?}***

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
```

Reference [./src/demo02.rs](./src/demo02.rs)

#### ***1.2.2 - Display***

```fmt::Debug``` hardly look compact and clean, so it is often advantageous to customize the output appearance. This is done by manually implementing ```fmt::Display```, which uses the ```{}``` print marker. Implement it look like this: 

```rust
// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}
```

```fmt::Display``` may be cleaner than ```fmt::Debug``` but this presents a problem for the std library. How should ambiguous types be displayed? For example, if the std library implemented a single style for all ```Vec<T>```, what style should it be? Would it be either of these two?

* ```Vec<path>```: ```/:/etc/home/username:/bin``` (split on ```:```)
* ```Vec<number>```: ```1, 2, 3``` (split on ```,```)

No, because there is no ideal style for all types and the ```std``` library doesn't presume to dictate one. ```fmt::Display``` is not implemented for ```Vec<T>``` or for any other generic containers. ```fmt::Debug``` must then be used for these for generic cases.

This is not a problem though because for any new container type which is not generic, ```fmt::Display``` can be implemented.

```rust
use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

So, ```fmt::Display``` has been implemented but ```fmt::Binary``` has not, and therefore cannot be used. ```std::fmt``` has many such ```traits``` and each requires its own implementation. This is detailed further in ```std::fmt```.

**Activity**

After checking the output of the above example, use the ```Point2D``` struct as a guide to add a ```Complex``` struct to the example. When printed in the same way, the output should be:

```shell
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, image: 7.2 }
```

**Testcase: List**

Implementing ```fmt::Display``` for a structure where the elements must each be handled sequentially is tricky. The problem is that each ```write!``` generates a ```fmt::Result```. Proper handling of this requires dealing with *all* the results. Rust provides the ```?``` operator for exactly this purpose.

Using ```?``` on ```write!``` looks like this:

```rust
// Try `write!` to see if it errors.
// If it errors, return the error. Otherwise continue
write!(f, "{}", value)?;
```

With ```?``` available, implementing ```fmt::Display``` for a ```Vec``` is straightforward:

```rust
use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]");
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

**Activity**

Try changing the program so that the index of each element in the vector is also printed. The new output should look like this:

```rust
[0: 1, 1: 2, 2: 3]
```

#### ***1.2.3 - Formatting***

We've seen that formatting is specified via a *format string*:

* ```format!("{}", foo) -> "3735928559"```
* ```format!(0x{:X}, foo) -> "0xDEADBEEF"```
* ```format!(0o{:o}, foo) -> "0o33653337357"```

The same variable (```foo```) can be formatted differently depending on which *argument type* is used: ```x``` vs ```o``` vs ```unspecified```.

This formatting functionally is implemented via trails, and there is one trait for each argument type. The most common formatting trait is ```Display```, which handles cases where the argument type is left unspecified ```{}``` for instance.

```rust
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }
}
```

**Activity**

Add an implementation of the ```fmt::Display``` trait for the ```Color``` struct above so that the output displays as: 

```shell
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

Reference [./src/demo03.rs](./src/demo03.rs)

## **2 - Primitives**

Rust provides access to a wide variety of ```primitives```. A sample includes:

**Scalar types**

* Signed integers: ```i8```, ```i16```, ```i32```, ```i64```, ```i128``` and ```isize``` (pointer size)
* Unsigned integer: ```u8```, ```u16```, ```u32```, ```u64```, ````u128```` and ```usize``` (pointer size)
* Floating point: ```f32```, ```f64```
* ```char``` Unicode scalar values like ```'a'```, ```'⍺'```, ```'∞'``` (4 bytes each)
* ```bool``` either ```true``` or ```false```
* and the unit type ```()```, whose only possible value is an empty tuple: ```()```

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

**Compound Types**

* arrays like ```[1, 2, 3]```
* tuples like ```(1, true)```

Variables can always be *type annotated*. Numbers may additionally be annotated via a *suffix or by default*.

Integer default to ```i32``` and floats to ```f64```.

Note that Rust can also infer types from context.

```rust
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
}
```

Reference [./src/demo04.rs](./src/demo04.rs)

### ***2.1 - Literals and Operator***

Integer ```1```, floats ```1.2```, characters ```'a'```, string ```"abc"```, booleans ```true``` and the unit type ```()``` can be expressed using literals. 

Integers can, alternatively, be expressed hexadecimal, octal or binary notation using these prefixes respectively: ```0x```, ```0o``` or ```0b```.

Underscores can be inserted in numeric literals to improve readability, e.g. ```1_0000``` is the same as ```1000```, and ```0.000_001``` is the same as ```0.000001```.

We need to tell the compiler the type of the literals we use. For now, we'll use the ```u32``` suffix to indicate that the literal is an unsigned 32-bit integer, and the ```i32``` suffix to indicate that it's signed 32-bit integer.

The operators available and their precedence in Rust are similar to other C-like languages.

```rust
fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
```

Reference [./src/demo05.rs](./src/demo05.rs)

### ***2.2 - Tuples***

A tuple is a collection of values of different types. Tuples are constructed using parentheses ```()```, and each tuple itself is a value with type signature ```(T1, T2, ...)```, where ```T1```, ```T2``` are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.

```rust
// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

}
```

Reference [./src/demo06.rs](./src/demo06.rs)

**Activity**

1. *Recap*: Add the ```fmt::Display``` trait the ```Matrix``` struct in the above example, so that if you switch from printing the debug format ```{:?}``` to the display format ```{}```, you see the following output:

```shell
( 1.1, 1.2 )
( 2.1, 2.2 )
```

2. Add ```transpose``` function using the ```reverse``` function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:

```rust
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
```

results in the output: 

```shell
Matrix:
( 1.1, 1.2 )
( 2.1, 2.2 )
Transpose:
( 1.1, 2.1 )
( 1.2, 2.2 )
```

### ***2.3 - Arrays and Slices***

An array is a collection of objects of the same type ```T```, stored in contiguous memory. Arrays are created using brackets ```[]```, and their length. which is know at compile time, is part of their type signature ```[T; length]```.

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture e.g. 65-bits on an x86-64. Slices can be used to borrow a section of an array, and have the type signature ```&[T]```.

```rust
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}
```

Reference [./src/demo07.rs](./src/demo07.rs)

## **3 - Custom Types**

Rust custom data types are formed mainly through the two keywords: 

* ```struct```: define a structure
* ```enum```: define an enumeration

Constants can also be created via the ```const``` and ```static``` keywords.

### ***3.1 - Structures***

There are three types of structures ("structs") that can be created using the ```struct``` keyword:

* Tuple structs, which are, basically, named tuples. 
* The classic C-structs
* Unit structs, which are field-less, are useful for generics.

```rust
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

Reference [./src/demo08.rs](./src/demo08.rs)

**Activity**

1. Add a function ```rect_area``` which calculates the area of a ```Rectangle``` (try using nested destructing). 
2. Add a function ```square``` which takes a ```Point``` and a ```f32``` as arguments, and returns a ```Rectangle``` with its top left corner on the point, and a width and height corresponding to the ```f32```.

### ***3.2 - Enums***

The ```enum``` keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as as ```struct``` is also valid as an ```enum```.

```rust
// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```

Reference [./src/demo09.rs](./src/demo09.rs)

**Type aliases** 

If you use a type alias, you can refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it.

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}
```

The most common place you'll see this is ```impl``` blocks using the ```Self``` alias.

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

#### ***3.2.1 - use***

The ```use``` declaration can be used so manual scoping isn't needed

```rust
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
```

Reference [./src/demo10.rs](./src/demo10.rs)

#### ***3.2.2 - C-like***

```enum``` can also be used as C-like enums

```rust
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

Reference [./src/demo11.rs](./src/demo11.rs)

#### ***3.2.3 - Testcase: linked-list***

A common way to implement a linked-list is via ```enum```

```rust
use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

Reference [./src/demo12.rs](./src/demo12.rs)

### ***3.3 - Constants***

Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation: 

* ```const```: An unchangeable value (the common case). 
* ```static```: A possibly ```mut``` able variable with ```'static``` lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is ```unsafe```.

```rust
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
```

Reference [./src/demo13.rs](./src/demo13.rs)

## **4 - Variable Bindings**

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the ```let``` binding.

```rust
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
}
```

Reference [./src/demo14.rs](./src/demo14.rs)

### ***4.1 - Mutability***

Variable bindings are immutable by default, but this can be overridden using the ```mut``` modifier.

```rust
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    _immutable_binding += 1;
    // FIXME ^ Comment out this line
}
```

Reference [./src/demo15.rs](./src/demo15.rs)

### ***4.2 - Scope and Shadowing***

Variable bindings have a scope, and are constrained to live in a *block*. A block is a collection of statements enclosed by braces ```{}```.

```rust
fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}
```

Reference [./src/demo16.rs](./src/demo16.rs)

Also, variable shadowing is allowed

```rust
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
```

Reference [./src/demo17.rs](./src/demo17.rs)

### ***4.3 - Declare first***

It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.

```rust
fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
```

Reference [./src/demo18.rs](./src/demo18.rs)

### ***4.4 - Freezing***

When data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutable binding goes out of scope:

```rust
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
```

Reference [./src/demo19.rs](./src/demo19.rs)

## **5 - Types**

Rust provides several mechanisms to change or define the type of primitive and user defined types.

The following sections cover: 

* ```Casting``` between primitive types 
* Specifying the desired type of ```literals``` 
* Using ```type inference```
* ```Aliasing``` types

### ***5.1 - Casting***

Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the ```as``` keyword.

Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust.

```rust
// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules. 
    // A float cannot be directly converted to a char.
    let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    
    // 128 as i8 -> -128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
    
    // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
    // when casting from float to int. If the floating point value exceeds 
    // the upper bound or is less than the lower bound, the returned value 
    // will be equal to the bound crossed.
    
    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);
    
    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```

Reference [./src/demo20.rs](./src/demo20.rs)
### ***5.2 - Literals***

Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that the literal ```42``` should have the type ```i32```, write ```42i32```.

The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists, the compiler will use ```i32``` for integer, and ```f64``` for floating-point numbers.

```rust
fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

Reference [./src/demo21.rs](./src/demo21.rs)

There are some concepts used in the previous code that haven't been explained yet, here's a brief explanation for the impatient readers: 

* ```std::mem::size_of_val``` is a function, but called with its *full path*. Code can be split in logical units called *modules*. In this case, the ```size_of_val``` function is defined in the ```mem``` module, and the ```mem``` module is defined in the ```std``` crate. For more details, see modules and crates.

### ***5.3 - Inference***

The type inference engine is pretty smart. It does more than looking at the type of the value expression during an initialization. It also looks at how the variable is used afterwards to infer its type. Here's an advanced example of type inference.

```rust
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
```

Reference [./src/demo22.rs](./src/demo22.rs)

No type annotation of variables was needed, the compiler is happy and so is the programmer!

### ***5.4 - Aliasing***

The ```type``` statement can be used to give a new name to an existing type. Types must have ```UpperCamelCase``` names, or the compiler will raise a warning. The exception to this rule are the primitive types: ```usize```, ```f32```, etc.

```rust
// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
```

Reference [./src/demo23.rs](./src/demo23.rs)

The main use of aliases is to reduce boilerplate; for example the ```IoResult<T>``` type is an alias for the ```Result<T, IoError>``` type.

## **6 - Conversion**

Primitive types can be converted to each other through ```casting```.

Rust addresses conversion between custom types (i.e., ```struct``` and ```enum```) by the use of ```traits```. The generic conversions will use the ```From``` and ```Into``` traits. 

However there are more specific ones for the more common cases, in particular when converting to and from ```String S```.

### ***6.1 - From and Into***

The ```From``` and ```Into``` traits are inherently linked, and this is actually part of its implementation. If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.

**From**

The ```From``` trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types. There are numerous implementations of this trait within the standard library for conversion of primitive and common types.

For example we can easily convert a ```str``` into a ```String```

```rust
let my_str = "hello";
let my_string = String::from(my_str);
```

We can do similar for defining a conversion for our own type.

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

Reference [./src/demo24.rs](./src/demo24.rs)

**Into** 

The ```Into``` trait is simply the reciprocal of the ```From``` trait. That is, if you have implemented the ```From``` trait for your type, ```Into``` will call it when necessary.

Using the ```Into``` trait will typically require specification of the type to convert into as the compiler is unable to determine this most of the time. However this is a small trade-off considering we get the functionality for free.

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

Reference [./src/demo25.rs](./src/demo25.rs)

### ***6.2 - TryFrom and TryInto***

Similar to ```From``` and ```Into```, ```TryFrom``` and ```TryInto``` are generic traits for converting between types. Unlike ```From/Into```, the ```TryFrom/TryInto``` traits are used for fallible conversions, and as such, return ```Result S```.

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

Reference [./src/demo26.rs](./src/demo26.rs)

### ***6.3 - To and From Strings***

**Converting to String** 

To convert any type to a ```String``` is as simple as implementing the ```ToString``` trait for the type. Rather than doing so directly, you should implement the ```fmt::Display``` trait which automagically provides ```ToString``` and also allows printing the type as discussed in the section on ```print!```.

```rust
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

Reference [./src/demo27.rs](./src/demo27.rs)

**Parsing a String** 

One of the more common types to convert a string into is a number. The idiomatic approach to this is to use the ```parse``` function and either to arrange for type inference or to specify the type to parse using the 'turbofish' syntax. Both alternatives are shown in the following example.

This will convert the string into the type specified as long as the ```FromStr``` trait is implemented for that type. This is implemented for numerous types within the standard library. To obtain this functionality on a user defined type simply implement the ```FromStr``` trait for that type.

```rust
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
```

Reference [./src/demo28.rs](./src/demo28.rs)

## **7 - Expressions**

## **8 - Flow of Control**

## **9 - Functions**

## **10 - Modules**

## **11 - Crates**

## **12 - Cargo**

## **13 - Attributes**

## **14 - Generics**

## **15 - Scoping rules**

## **16 - Traits**

## **17 - Macros**

## **18 - Error handling**

## **19 - Std library types**

## **20 - Std misc**

## **21 - Testing**

## **22 - Unsafe Operations**
## **23 - Compatibility**

## **24 - Meta**