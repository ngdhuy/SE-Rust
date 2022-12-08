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

A Rust proggram is (mostly) made up of a series of statements

```rust
fn main() {
    // statement
    // statement
    // statement
}
```

There are a few kinds of statements in Rust. The most common two are declaring a variable binding, and use a ```;``` with expression: 

```rust
fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
}
```

Blocks are expressions too, so they can be used as values in assignments. The last expression in the block will be assigned to the place expression such as as local variable. However, if the last expression of the block ends with a semicolon, the return value will be ```()```.

```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```

Reference [./src/demo29.rs](./src/demo29.rs)

## **8 - Flow of Control**

An integral part of any programming language are ways to modify control flow: ```if/else```, ```for```, and others. Let's talk about them in Rust.

### ***8.1 - if/else***

Branching with ```if-else``` is similar to other languages. Unlike many of them, the boolean condition doesn't need to be surrounded by parentheses, and each condition is followed by a block. ```if-else``` conditionals are expressions, and, all branches must return the same type.

```rust
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}
```

Reference [./src/demo30.rs](./src/demo30.rs)

### ***8.2 - loop***

Rust provides a ```loop``` keyword to indicate an infinite loop.

The ```break``` statement can be used to exit a loop at anytime, whereas the ```continue``` statement can be used to skip the rest of the iteration and start a new one.

```rust
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
```

Reference [./src/demo31.rs](./src/demo31.rs)

**Nesting and Labels**

It's possible to ```break``` or ```continue``` outer loops when dealing with nested loops. In these cases, the loops must be annotated with some ```'label```, and the label must be passed to the ```break/continue``` statement.

```rust
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
```

Reference [./src/demo32.rs](./src/demo32.rs)

**Returning from loops**

One of the uses of a ```loop``` is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the ```break```, and it will be returned by the ```loop``` expression.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

Reference [./src/demo33.rs](./src/demo33.rs)

### ***8.3 - while***

The while keyword can be used to run a loop while a condition is true.

Let's write the infamous ```FizzBuzz``` using a while loop.

```rust
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}
```

Reference [./src/demo34.rs](./src/demo34.rs)

### ***8.4 - for and range***

**for and range** 

The ```for in``` construct can be used to iterate through an ```Iterator```. One of the easiest ways to create an iterator is to use the range notation ```a..b```. This yields values from ```a``` (inclusive) to ```b``` (exclusive) in steps of one.

Let's write FizzBuzz using ```for``` instead of ```while```.

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

Reference [./src/demo35.rs](./src/demo35.rs)

Alternatively, ```a..=b``` can be used for a range that is inclusive on both ends. The above can be written as:

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

Reference [./src/demo36.rs](./src/demo36.rs)

**for and iterators**

The for in construct is able to interact with an ```Iterator``` in several ways. As discussed in the section on the ```Iterator``` trait, by default the ```for``` loop will apply the ```into_iter``` function to the collection. However, this is not the only means of converting collections into iterators.

```into_iter```, ```iter``` and ```iter_mut``` all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.

* ```iter``` - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}
```

Reference [./src/demo37.rs](./src/demo37.rs)

* ```into_iter``` - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
```

Reference [./src/demo38.rs](./src/demo38.rs)

* ```iter_mut``` - This mutably borrows each element of the collection, allowing for the collection to be modified in 
place.

```rust
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```

Reference [./src/demo39.rs](./src/demo39.rs)

In the above snippets note the type of match branch, that is the key difference in the types of iteration. The difference in type then of course implies differing actions that are able to be performed.

### ***8.5 - match***

Rust provides pattern matching via the match keyword, which can be used link C ```switch```. The first matching arm is evaluated and all possible values must be covered.

```rust
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
```

Reference [./src/demo40.rs](./src/demo40.rs)

#### ***8.5.1 - Destructuring***

A ```match``` block can destructure items in a variety of ways.

* Destructuring Tuples
* Destructuring Arrays and Slices
* Destructuring Enums
* Destructuring Pointers
* Destructuring Structures

##### ***8.5.1.1 - Destructuring Tuples***

Tuples can be destructured in a ```match``` as follows:

```rust
fn main() {
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}
```

Reference [./src/demo41.rs](./src/demo41.rs)

##### ***8.5.1.2 - Destructuring Arrays and Slices***

Like ```tuples```, ```arrays``` and ```slices``` can be destructured this way:

```rust
fn main() {
    // Try changing the values in the array, or make it a slice!
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
```

Reference [./src/demo42.rs](./src/demo42.rs)
##### ***8.5.1.3 - Destructuring Enums***

An ```enum``` is destructured similarly

```rust
// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}
```

Reference [./src/demo43.rs](./src/demo43.rs)

##### ***8.5.1.4 - Destructuring Pointers/Ref***

For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts which are used differently from languages like C/c++

* Dereferencing ```*``` 
* Destructuring uses ```&```, ```ref```, and ```ref mut```

```rust
fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
```

Reference [./src/demo44.rs](./src/demo44.rs)

##### ***8.5.1.5 - Destructuring Structures***

Similarly, a ```struct``` can be destructured as shown

```rust
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
```

Reference [./src/demo45.rs](./src/demo45.rs)

#### ***8.5.2 - Guards***

A ```match``` *guard* can be added to filter the arm.

```rust
enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 Farenheit", t),
        Temperature::Farenheit(t) => println!("{}F is below 86 Farenheit", t),
    }
}
```

Reference [./src/demo46.rs](./src/demo46.rs)

Note that the compiler won't take guard conditions into account when checking if all patterns are covered by the match expression

```rust
fn main() {
    let number: u8 = 4; 

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // _ => unreachable!("Should never happen."), 
        // TODO ^ uncomment to fix compilation
    }
}
```

Reference [./src/demo57.rs](./src/demo47.rs)

#### ***8.5.3 - Binding***

Indirectly accessing a variable makes it impossible to branch and use that variable without rebinding. ```match``` provides the ```@``` sigil for binding values to names

```rust
// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```

Reference [./src/demo48.rs](./src/demo48.rs)

You can also use binding to "destructure" ```enum``` variants, such as ```Option```

```rust
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }
}
```

Reference [./src/demo49.rs](./src/demo49.rs)

### ***8.6 - if let***

For some use cases, when matching enums, ```match``` is awkward. For example

```rust
#![allow(unused)]
fn main() {
// Make `optional` of type `Option<i32>`
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
};

}
```

```shell
This is a really long string and `7`
```

```if let``` is cleaner for this use case and in addition allows various failure options to be specified

```rust
fn main() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
```

Reference [./src/demo50.rs](./src/demo50.rs)

In the same way, ```if let``` can be used to match any enum value

```rust
// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
```

Reference [./src/demo51.rs](./src/demo51.rs)

Another benefit is that ```if let``` allows us to match non-parameterized enum variants. This is true even in cases where the enum doesn't implement or derive ```PartialEq```. In such cases ```if Foo::Bar == a``` would fail to compile, because instances of the enum cannot be equated, however ```if let``` will continue to work.

Would you like a challenge? Fix following example to use ```if let```

```rust
// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if Foo::Bar == a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}
```

Reference [./src/demo52.rs](./src/demo52.rs)

### ***8.7 - while let***

Similar to ```if let```, ```while let``` can make awkward ```match``` sequences more tolerable. Consider the following sequence that increments ```i```

```rust
#![allow(unused)]
fn main() {
// Make `optional` of type `Option<i32>`
let mut optional = Some(0);

// Repeatedly try this test.
loop {
    match optional {
        // If `optional` destructures, evaluate the block.
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ Requires 3 indentations!
        },
        // Quit the loop when the destructure fails:
        _ => { break; }
        // ^ Why should this be required? There must be a better way!
    }
}
}
```

```shell
`i` is `0`. Try again.
`i` is `1`. Try again.
`i` is `2`. Try again.
`i` is `3`. Try again.
`i` is `4`. Try again.
`i` is `5`. Try again.
`i` is `6`. Try again.
`i` is `7`. Try again.
`i` is `8`. Try again.
`i` is `9`. Try again.
Greater than 9, quit!
```

Using ```while let``` makes this sequence much nicer

```rust
fn main() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}
```

Reference [./src/demo53.rs](./src/demo53.rs)

## **9 - Functions**

Function are declared using the ```fn``` keyword. Its arguments are type annotated, just like variables, and, if the function returns a value, the return type must be specified after an arrow ```->```. 

The final expression in the function will be used as return value. Alternatively, the ```return``` statement can be used to return a value earlier from within the function, even from inside loops or ```if``` statement.

Let's rewrite FizzBuzz using function!

```rust
// Unlike C/C++, there's no restriction on the order of function definitions
fn main() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
```

Reference [./src/demo54.rs](./src/demo54.rs)

### ***9.1 - Methods***

**Associated functions & Methods**

Some functions are connected to a particular type. These come in two forms: associated functions, and methods. Associated functions are functions that are defined on a type generally, while methods are associated functions that are called on a particular instance of type.

```rust
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();
    // TODO ^ Try uncommenting this line
}
```

Reference [./src/demo55.rs](./src/demo55.rs)

### ***9.2 - Closures***

Closures are functions that can capture the enclosing environment. For example, a closure that captures the ```x``` variable:

```shell
|val| val + x
```

The syntax and capabilities of closures make them very convenient for on the fly usage. Calling a closure is exactly like calling a function. However, both input and return types *can* be inferred and input variable names *must* be specified.

Other characteristics of closures include: 

* Using ```||``` instead of ```()``` around input variables. 
* Optional body delimination ( ```{}``` ) for a single expression (mandatory otherwise). 
* The ability to capture the outer environment variables.

```rust
fn main() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

}
```

Reference [./src/demo56.rs](./src/demo56.rs)

#### ***9.2.1 - Capturing***

Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables:

* by reference: ```&T```
* by mutable reference: ```&mut T```
* by value: ```T```

They preferentially capture variables by reference and only go lower when required.

```rust
fn main() {
    use std::mem;
    
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time. 
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count; 
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 

    
    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}
```

Reference [./src/demo57.rs](./src/demo57.rs)

Using ```move``` before vertical pipes forces closure to take ownership of captured variables

```rust
fn main() {
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}
```

Reference [./src/demo58.rs](./src/demo58.rs)

#### ***9.2.2 - As input parameters***

While Rust chooses how to capture variables on the fly mostly without type annotation, this ambiguity is not allowed when writing functions. When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few ```traits```, and they're determined by what the closure does with captured value. In order of decreasing restriction, they are:

* ```Fn```: the closure uses the captured value by reference (```&T```)
* ```FnMut```: the closure uses the captured value by mutable reference (```&mut T```)
* ```FnOnce```: the closure uses the captured value by value (```T```)

On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

For instance, consider a parameter annotated as ```FnOnce```. This specifies that the closure may capture by ```&T```, ```&mut T```, or ```T```, but the compiler will ultimately choose based on how the captured variables are used in the closure.

This is because if a move is possible, then any type of borrow should also be possible. Note that the reverse is not true. If the parameter is annotated as ```Fn```, then capturing variables by ```&mut T``` or ```T``` are not allowed.

In the following example, try swapping the usage of ```Fn```, ```FnMut```, and ```FnOnce``` to see what happens:

```rust
// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

Reference [./src/demo59.rs](./src/demo59.rs)

#### ***9.2.3 - Type anonymity***

Closures succinctly capture variables from enclosing scopes. Does this have any consequences? It surely does. Observe how using a closure as a function parameter requires ```generics```, which is necessary because of how they are defined:

```rust
// `F` must be generic.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, meanwhile implementing the functionality via one of the traits: ```Fn```, ```FnMut```, or ```FnOnce``` for this unknown type. This type is assigned to the variable which is stored until calling.

Since this new type is of unknown type, any usage in a function will require generics. However, an unbounded type parameter ```<T>``` would still be ambiguous and not be allowed. Thus, bounding by one of the traits: ```Fn```, ```FnMut```, or ```FnOnce``` (which it implements) is sufficient to specify its type.

```rust
// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}
```

Reference [./src/demo60.rs](./src/demo60.rs)

#### ***9.2.4 - Input functions***

Since closures may be used as arguments, you might wonder if the same can be said about functions. And indeed they can! If you declare a function that takes a closure as parameter, then any function that satisfies the trait bound of that closure can be passed as a parameter.

```rust
// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

Reference [./src/demo61.rs](./src/demo61.rs)

As an additional note, the ```Fn```, ```FnMut```, and ```FnOnce``` traits dictate how a closure captures variables from the enclosing scope.

#### ***9.2.5 - As output parameters***

Closures as input parameters are possible, so returning closures as output parameters should also be possible. However, anonymous closure types are, by definition, unknown, so we have to use ```impl``` Trait to return them.

The valid traits for returning a closure are:

* ```Fn```
* ```FnMut```
* ```FnOnce```

Beyond this, the ```move``` keyword must be used, which signals that all captures occur by value. This is required because any captures by reference would be dropped as soon as the function exited, leaving invalid references in the closure.

```rust
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
```

Reference [./src/demo62.rs](./src/demo62.rs)

#### ***9.2.6 - Example in std***

This section contains a few examples of using closures from the ```std``` library.

##### ***9.2.6.1 - Iterator::any***

```Iterator::any``` is a function which when passed an iterator, will return true if any element satisfies the predicate. Otherwise ```false```. Its signature:


```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool;
}
```

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors.

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
```

Reference [./src/demo63.rs](./src/demo63.rs)

##### ***9.2.6.2 - Searching through iterators***

```Iterator::find``` is a function which iterates over an iterator and searches for the first value which satisfies some condition. If none of the values satisfy the condition, it returns ```None```. Its signature:

```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}
```

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}
```

Reference [/src/demo64.rs](./src/demo64.rs)

```Iterator::find``` gives you a reference to the item. But if you want the *index* of the item, use ```Iterator::position```.

```rust
fn main() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
    // we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
    // we do not have to destructure    
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
```

Reference [./src/demo65.rs](./src/demo65.rs)

See also: 

```rust
std::iter::Iterator::find
std::iter::Iterator::find_map
std::iter::iterator::position
std::iter::Iterator::rposition
```
### ***9.3 - Higher Order Functions***

Rust provides Higher Order Functions (HOF). These are functions that take one or more functions and/or produce a more useful function. HOFs and lazy iterators give Rust its functional flavor.

```rust
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```

Reference [./src/demo66.rs](./src/demo66.rs)

```Option``` and ```Iterator``` implement their fair share of HOFs

### ***9.4 - Diverging Functions***

Diverging functions never return. They are marked using ```!```, which is an empty type.

```rust
fn foo() -> ! {
    panic!("This call never returns.");
}
```

As opposed to all the other types, this one cannot be instantiated, because the set of all possible values this type can have is empty. Note that, it is different from the ```()``` type, which has exactly one possible value.

For example, this function returns as usual, although there is no information in the return value.

```rust
fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.");
}
```

As opposed to this function, which will never return the control back to the caller.

```rust
#![feture(newver_type)]

fn main() {
    let x: ! = panic("This call never return.");
    println!("You will never see this line!");
}
```

Although this might seem like an abstract concept, it is in fact very useful and often handy. The main advantage of this type is that it can be cast to any other one and therefore used at places where an exact type is required, for instance in ```match``` branches. This allows us to write code like this:

```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```

Reference [./src/demo67.rs](./src/demo67.rs)

It is also return type of functions that loop forever (e.g. ```loop {}```) like network servers or functions that terminate the process (e.g. ```exit()```).

## **10 - Modules**

Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: function, structs, straits, ```impl``` blocks, and even other modules.

### ***10.1 - Visibility***

By default, the items in a module have private visibility, but this can be overridden with the ```pub``` modifier. Only the public items of a module can be accessed from outside the module scope.

```rust
// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}
```

Reference [./src/demo68.rs](./src/demo68.rs)

### ***10.2 - Struct Visibility***

Structs have an extra level of visibility with their fields. The visibility defaults to private, and can be overridden with the ```pub``` modifier. This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation).

```rust
mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
```

Reference [./src/demo69.rs](./src/demo69.rs)

### ***10.3 - The use declaration***

The ```use``` declaration can be used to bind a full path to a new name, for easier access. It is often used like this:

```rust
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}
```

Reference [./src/demo70.rs](./src/demo70.rs)

You can use the ```as``` keyword to bind imports to a different name:

```rust
// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}
```

Reference [./src/demo71.rs](./src/demo71.rs)

### ***10.4 - Super and self***

The ```super``` and ```self``` keywords can be used in the path to remove ambiguity when accessing items and to prevent unnecessary hardcoding of paths.

```rust
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

Reference [./src/demo72.rs](./src/demo72.rs)

### ***10.5 - File hierarchy***

**File Hierarchy** 

Modules can be mapped to a file/directory hierarchy. Let's break down the visibility example in files: 

```shell
$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs
```

In ```split.rs```

```rust
// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
```

In ```my.rs```

```rust
// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```

In ```my/nested.rs```

```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

In ```my/inaccessible.rs```

```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

Let's check that things still work as before

```shell
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

## **11 - Crates**

A crate is a compilation unit in Rust. Whenever ```rustc some_file.rs``` is called, ```some_file.rs``` is treated as the crate file. If ```some_file.rs``` has ```mod``` declarations in it, then the contents of the module files would be inserted in places where ```mod``` declarations in the crate file are found, before running the compiler over it. In other words, modules do not get compiled individually, only crates get compiled.

A crate can be compiled into a binary or into a library. By default, ```rustc``` will produce a binary from a crate. This behavior can be overridden by passing the ```--crate-type``` flag to ```lib```.

### ***11.1 - Creating a Library***

Let's create a library, and then see how to link it to another crate.

```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

```shell
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

Libraries get prefixed with "lib", and by default they get named after their crate file, but this default name can be overridden by passing the ```--crate-name``` option to ```rustc``` or by using ```crate_name attribute```
### ***11.2 - Using a Library***

To link a crate to this new library you may use ```rusts```'s ```--extern``` flag. All of its items will then be imported under a module named the same as the library. This module generally behaves the same way as any other module.

```rust
// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
```

```shell
# Where library.rlib is the path to the compiled library, assumed that it's
# in the same directory here:
$ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```

## **12 - Cargo**

```cargo``` is the official Rust package management tool. It has lots of really useful features to improve code quality and developer velocity! The include. 

* Dependency management and integration with [crates.io](https://crates.io) (the official Rust package registry). 
* Awareness of unit tests. 
* Awareness of benchmarks.

This chapter will go through some quick basics, but you can find the comprehensive docs in [The Cargo Book](https://doc.rust-lang.org/cargo/)

### ***12.1 - Dependencies***

Most programs have dependencies on some libraries. If you have ever managed dependencies by hand, you know how much of a pain this can be. Luckily, the Rust ecosystem comes standard with ```cargo```! ```cargo``` can mange dependencies for a project.

To create a new Rust project

```shell
# A binary
cargo new foo

# OR A library
cargo new --lib foo
```

For the rest of this chapter, let's assume we are making a binary, rather than a library, but all of the concepts are the same. 

After the above commands, you should see a file hierarchy like this: 

```shell
foo
├── Cargo.toml
└── src
    └── main.rs
```

The ```main.rs``` is the root source file for your new project -- nothing new there. The ```Cargo.toml``` is the config file for ```cargo``` fro this project (```foo```). If you look inside it, you should see something like this: 

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

The ```name``` field under ```[package]``` determines the name of the project. This is used by ```crates.io``` if you public the crate (more later). It is also the name of the output binary when you compile. 

The ```version``` field is a crate version number using [Semantic Versioning](https://semver.org)

The ```authors``` field is a list of authors used when publishing the crate. 

The ```[dependencies]``` section lets you add dependencies for your project.

For example, suppose that we want out program to have a great CLI. You can find lots of great packages on ```crates.io``` (the official Rust package registry). One popular choice is ```clap```. As of this writing, the most recent published version of ```clap``` is ```2.27.1```. To add a dependency to our program, we can simply add the following to our ```Cargo.toml``` under ```[dependencies]: clap = "2.27.1"```. And that's it! You can start using ```clap``` in our program.

```cargo``` also supports ```other types of dependencies```. Here is just a small smapling:

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem
```

```cargo``` is more than a dependency manager. All of the available configuration options are listed in the [format specification](https://doc.rust-lang.org/cargo/reference/manifest.html) of ```Cargo.toml```

To build our project we can execute ```cargo build``` anywhere in the project director (including subdirectories!). We can also do ```cargo run``` to build and run. Notice that these commands will resolve all dependencies, download crates if needed, and build everything, including your crate. (Note that it only rebuilds what it has not already built, similar to ```make```). 

Voila! That's all there is to it!

### ***12.2 - Conventions***

In the previous chapter, we saw the following directory hierarchy: 

```shell
foo
├── Cargo.toml
└── src
    └── main.rs
```

Suppose that we wanted to have two binaries in the same project, though. What then?

It turns out that ```cargo``` supports this. The default binary name is ```main```, as we saw before, but you can add additional by placing them in a ```bin/``` directory:

```shell
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

To tell ```cargo``` to compile or run this binary as opposed to the default or other binaries, we just pass ```cargo``` the ```--bin my_other_bin``` flag, where ```my_other_bin``` is the name of the binary we want to work with. 

In addition to extra binaries, ```cargo``` supports [more features](https://doc.rust-lang.org/cargo/guide/project-layout.html) such as benchmarks, tests, and example.

In the next chapter, we will look more closely at tests.
### ***12.3 - Tests***

As we know testing is integral to piece of software! Rust has first-class support for unit and integration testing ([see this chapter](https://doc.rust-lang.org/book/ch11-00-testing.html) in TRPL).

From the testing chapters linked above, we see how to write unit tests and integration tests. Organizationally, we can place unit tests in the modules they test and integration tests in their own ```test/``` directory:

```shell
foo
├── Cargo.toml
├── src
│   └── main.rs
│   └── lib.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

Each file in ```tests``` is a separate ```integration test```, i.e. a test that is meant to test your library as if it were being called from a dependent crate. 

The Testing chapter elaborates on the three different testing styles: Unit, Doc and Integration. 

```cargo``` naturally provides an easy way to run all of your test!

```shell
$ cargo test
```

You should see output like this;

```shell
$ cargo test
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 3 tests
test test_bar ... ok
test test_baz ... ok
test test_foo_bar ... ok
test test_foo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

One word of caution: Cargo may run multiple tests concurently, so make sure that they don't race with each other.

One example of this concurency causing issues is if two tests output to a file, such as below:

```rust
#![allow(unused)]
fn main() {
    #[cfg(test)]
    mod tests {
        // Import the necessary modules
        use std::fs::OpenOptions;
        use std::io::Write;

        // This test writes to a file
        #[test]
        fn test_file() {
            // Opens the file ferris.txt or creates one if it doesn't exist.
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ferris.txt")
                .expect("Failed to open ferris.txt");

            // Print "Ferris" 5 times.
            for _ in 0..5 {
                file.write_all("Ferris\n".as_bytes())
                    .expect("Could not write to ferris.txt");
            }
        }

        // This test tries to write to the same file
        #[test]
        fn test_file_also() {
            // Opens the file ferris.txt or creates one if it doesn't exist.
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ferris.txt")
                .expect("Failed to open ferris.txt");

            // Print "Corro" 5 times.
            for _ in 0..5 {
                file.write_all("Corro\n".as_bytes())
                    .expect("Could not write to ferris.txt");
            }
        }
    }
}
```

Although the intent is to get the following

```shell
$ cat ferris.txt
Ferris
Ferris
Ferris
Ferris
Ferris
Corro
Corro
Corro
Corro
Corro
```

What actually gets put into ```ferris.txt``` is this

```shell
$ cargo test test_foo
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
```

### ***12.4 - Build Scripts***

**Buld Scripts**

Sometimes a normal build from ```cargo``` is not enough. Perhaps your crate needs some pre-requisites before ```cargo``` will successfully compile, thiings like code generation, or some antive code that needs to be compiled. To solve this problem we have build scripts that Cargo can run.

To add a build script to your package it can either be specified in the ```Cargo.toml``` as follows: 

```toml
[package]
...
build = "build.rs"
```
Otherwise Cargo will look for a ```build.rs``` file in the project directory by default.

**How to use a build script**

The build script is simply another Rust file that will be compiled and invoked prior to compiling anything else in the package. Hence it can be used to fulfill pre-requisites of your crate. 

Cargo provides the script with inputs via environment variables specified here that can be used. 

The script provides output via stdout. All lines printed are written to ```target/debug/build/<pkg>/output```. Further, lines prefixed with ```cargo:``` will be interpreted by Cargo directly and hence can be used to define parameters for the package's compilation. 

For further specification and example have a read of the [Cargo specifilecation](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

## **13 - Attributes**

An attribute is metadata applied to some module, crate or item. This metadataz can be used to/for: 

* Conditional compilation of code
* Set crate name, version and type (binary or library)
* Disable lints (warning)
* Enable compiler features (macros, glob imports, etc.)
* link to a foreign library
* Mark functions as unit tests
* Mark functions that will be part of a benchmark
* Attribute like macros

When attributes apply to a whole crate, their syntax is ```#![crate_attribute]```, and when they apply to a module or item, the syntax is ```#[item_attribute]``` (notice the missing bang ```!```). 

Attributes can take arguments with different syntaxes: 

* ```#[attribute = "value"]```
* ```#[attribute(key = "value")]```
* ```#[attribute(value)]```

Attributes can have multiple values and can be separated over multiple lines, too: 

```rust
#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]
```

### ***13.1 - dead_code***

The compiler provides a ```dead_code``` *lint* that will warn about unused functions. An *attribute* can be used to disable the lint. 

```rust
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}
```

Reference [./src/demo73.rs](./src/demo73.rs)

Note that in real programs, you should eliminate dead code. In these example we'll allow dead code in some places because of the interactive nature of the examples.

### ***13.2 - Crates***

The ```crate_type``` attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library), and the ```crate_name``` attribute can be used to set the name of the crate.

However, it is important to note that both the ```crate_type``` and ```crate_name``` attribute have no effect whatsoever when using Cargo, the Rust package manager. Since Cargo is used for the majority of Rust projects, this mean real-world uses of ```crate_type``` are relatively limited.

```rust
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

When the ```crate_type``` attribute is used, we no longer need to pass the ```--crate-type``` flag to ```rustc```.

```shell
$ rustc lib.rs
$ ls lib*
library.rlib
```

### ***13.3 - cfg***

Configuration conditional checks are possible through two differenct operators: 

* the ```cfg``` attribute: ```#[cfg(...)]``` in attribute position
* the ```cfg!``` macro: ```cfg!(...)``` in boolean expressions

While the former enable conditional compilation, the latter conditionaly evaluates to ```true``` or ```false``` literals allowing for checks at run-time. Both utillize identical argument syntax.

```cfg!```, unlike ```#[cfg]```, does not remove any code and only evaluates to true or false. For example, all blocks in an if/else expression need to be valid when ```cfg!``` is used for condition, regardless of what ```cfg!``` is evaluating.

```rust
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

Reference [./src/demo74.rs](./src/demo74.rs)

**Custom** 

Some conditionals like ```target_os``` are implicity provided by ```rustc```, but custom conditionals must be passed to ```rustc``` using the ```--cfg``` flag.

```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
```

Try to run this to see what happens without the custom ```cfg``` flag. 

With the custome ```cfg``` flag

```shell
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```

## **14 - Generics**

*Generics* is the topic of generalizing types and functionalities to broader cases. This is extremely useful for reducing code duplication in may ways, but can call for rather involved syntax. Namely, being generic requires talking greate care to specify over which types a generic type is actually considered valid. The simplest and most common use for generics is for type parameters. 

A type parameter is specified as generic by the use of angle brackets and uppder Camel Case: ```<Aaa, Bbb, ...>```. "Gerneric type parameter" are typically represented as ```<T>```. In Rust, "generic" also describes anything that accepts one or more generic type parameters ```<T>```. Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic). 

For example, defining a generic *function* name ```foo``` that takes an argument ```T``` of type type: 

```rust
fn foo<T>(arg: T) { ... }
```

Because ```T``` has been specified as a generic type parameter using ```<T>```, it is considered generic when used here as ```(arg: T)```. This is the case even if ```T``` has previously been defined as a ```struct```.

This example shows some of the syntax in action: 

```rust
// A concrete type `A`.
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);
    
    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`.
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}
```

Reference [./src/demo75.rs](./src/demo75.rs)

### ***14.1 - Functions***

The same set of rules can be applied to functions: a type ```T``` becomes generic when preceded by ```<T>```. 

Using generic functions sometimes requires explicitly specifying type parameter. This may be the case if the function is called where the return type is generic, or if the compiler doesn't have enough information to infer the necessary type parameters. 

A function call with explicity specified type parameters looks like: ```fun::<A, A, ...>()```.

```rust
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not 
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}
```

Refence [./src/demo76.rs](./src/demo76.rs)

### ***14.2 - Implementation***

Similar to functions, implementations require care to remain generic.

```rust
struct S; // Concrete type `S`
struct GenericVal<T>(T); // Generic type `GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}
```

```rust
struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
```

Reference [./src/demo77.rs](./src/demo77.rs)

### ***14.3 - Traits***

Of course ```trait``` s can also be generic. Here we define one which reimplements the ```Drop trait``` as a generic method to ```drop``` itself and an input.

```rust
// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}
```

Referecne [./src/demo78.rs](./src/demo78.rs)

### ***14.4 - Bounds***

When working with generics, the type parameters often must use traits as *bounds* to stipulate what functionality as type implements. Form example, the following example uses the trait ```Display``` to print and so it requires ```T``` to be bound ```Display```; that is, ```t``` *must* implement ```Display```.

```rust
// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Bounding restrics the generic to types that conform to be bounds. That is: 

```rust
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
let s = S(vec![1]);
```

Another effect of bounding is that generic instances are allowed to access the ```methods``` of traits specified in the bounds. For example:

```rust
// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    //print_debug(&_triangle);
    //println!("Area: {}", _triangle.area());
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`. 
}
```

Reference [./src/demo79.rs](./src/demo79.rs)

As an additional note, ```where``` clauses can also be used to apply bounds in some cases to be more expressive.

**Testcse: empty bounds**

A consequence of how bounds work is that even if a ```trait``` doesn't include any functionality, you can still use it as a bound. ```Eq``` and ```copy``` are examples of such ```trait``` s from the ```std``` library.

```rust
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
```

Reference [./src/demo80.rs](./src/demo80.rs)

### ***14.5 - Multiple bounds***

Multiple bounds for a single type can be applied with a ```+```. Like normal, different types are separated with ```,```.

```rust
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);
}
```

Reference [./src/demo81.rs](./src/demo81.rs)

### ***14.6 - Where clauses***

A bound can also be expressed using a ```where``` clause immediately before the opening ```{```, rather than at the type's first mention. Additionally, ```where``` clauses can apply bounds to arbitrary types, rather than just to type parameters. 

Some cases that a ```where``` clause is useful: 

```rust 
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* When using a ```where``` clause is more expressive than using normal syntax. The ```iml``` in this example cannot be directly expressed without a ```where``` clause: 

```rust
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or 
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
```

Reference [./src/demo82.rs](./src/demo82.rs)

### ***14.7 - New Type Idiom***

The ```newtype``` idiom gives compile tiem guarantees that the right type of value is supplied to a prgogram. 

For example, an age verification function that checks age in years, *must* be given a value of type ```Years```.

```rust
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}
```

Reference [./src/demo83.rs](./src/demo83.rs)

Uncomment the last print satement to observe that the type supplied must be ```Years```. 

to obtain the ```newtype``` 's value as the base type, you may use the tuple or destructuring syntax like so:

```rust
struct Years(i64);

fn main() {
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple
    let Years(years_as_primitive_2) = years; // Destructuring
}
```

### ***14.8 - Associated***

"Associated items" refers to a se of rules pertaining to ```item``` S of various types. It is an extension to ```trait``` generics, and allows ```trait```s to internally define new items. 

One such item is called an *associated type*, providing simpler usage patterns when the ```trait``` is generic over its container type.

#### ***14.8.1 - The problem***

A ```trait``` that is generic over its container type has type specification requirements - users of the ```trait``` *must* specify all of its generic types. 

In the example below, the ```Contains trait``` allows the use of the generic types ```A``` and ```B```. The ```trait``` is then implemented for the ```Container``` type, specifying ```i32``` for ```A``` and ```B``` so that it can be uesd with ```fn differenc()```. 

Because ```Contains``` is generic, we are forced to explicily state *all* of the generic types for ```fn difference()```. In paractice, we want a way to express that ```A``` and ```B``` are determined by the *input* ```C```. As you will see in the next section, associated types provide exactly that capbility.

```rust
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
```

Reference [./src/demo84.rs](./src/demo84.rs)

#### ***14.8.2 - Associated types***

The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as *output* types. Syntax for the ```trait``` definition is as follows: 

```rust
#![allow(unused)]
fn main() {
// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
}
```

Note that functions that use the ```trait Contains``` are not longer required to express ```A``` or ```B``` at all: 

```rust
// Without using associated types
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

Let's rewrite the example from the previous section using associated types

```rust
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    // Define generic types here which methods will be able to utilize.
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
```

Reference [./src/demo85.rs](./src/demo85.rs)

### ***14.9 - Phantom type parameters***

A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time. 

Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. These extra parameters hold no storage values, and have no runtime behavior. 

In the following example, we combine ```std:marker::PhantomData``` with the phantom type parameter concept to create tuples containing different data types.

 ```rust
use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: Storage is allocated for generic type `A`, but not for `B`.
//       Therefore, `B` cannot be used in computations.

fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_tuple1 == _tuple2 yields: {}",
    //           _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_struct1 == _struct2 yields: {}",
    //           _struct1 == _struct2);
}
 ```

 Reference [./src/demo86.rs](./src/demo86.rs)

 **Testcase: unit clarification** 

 A useful method of unit conversions can be examined by implementing ```Add``` with a phantom type parameter. The ```Add trait``` is examined bellow:

 ```rust
// This construction would impose: `Self + RHS = Output`
// where RHS defaults to Self if not specified in the implementation.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` must be `T<U>` so that `T<U> + T<U> = T<U>`.
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

The whole implementation: 

```rust
use std::ops::Add;
use std::marker::PhantomData;

/// Create void enumerations to define unit types.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` is a type with phantom type parameter `Unit`,
/// and is not generic over the length type (that is `f64`).
///
/// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// The `Add` trait defines the behavior of the `+` operator.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() returns a new `Length` struct containing the sum.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specifies `one_foot` to have phantom type parameter `Inch`.
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` has phantom type parameter `Mm`.
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    // Since `Length` implements `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Nonsensical operations fail as they should:
    // Compile-time Error: type mismatch.
    //let one_feter = one_foot + one_meter;
}
```

Reference [./src/demo87.rs](./src/demo87.rs)

## **15 - Scoping rules**

Scopes play an important part in ownership, borrowing, and lifetimes. That is, they indicate to the compiler when borrows are valid, when resources can be freed, and when variables are created or destroysed.

### ***15.1 - RAII***

Variables in Rust do more than just hold data in the stack: they also *own* resources, e.g. ```Box<T>``` owns memory in the heap. Rust enforces ```RAII``` (Resource Acquisition Is Initialization), so whenever an object goes out of scope, its destructor is called and its owned resources are freed.

This behavior shields against *resource leak bugs*, so you'll never have to manually free memory or worry about memory leaks again! Here's a quick showcase:

```rust
// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
```

Of course, we can double check for memory errors using ```valgrind```:

```shell
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

No leaks here!

**Destructor**

The notion of a destructor in Rust is provided through the ```Drop``` trait. The destructor is called when the resource goes out of scope. This trait is not required to be implemented for every type, only implement it for your type if you required its own destructor logic.

Run the below example to see how the ```Drop``` trait works. When the variable in the ```main``` function goes out of scrope the custom destructor will be invoked.

```rust
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

Reference [./src/demo88.rs](./src/demo88.rs)

### ***15.2 - Ownership and moves***

Because variables are in charge of freeing own resources, **resource can only have one owner**. This also prevents resources from being freed more than once. Note that not all variables own resources (e.g. references).

When doing assignments (```let x = y```) or passing function arguments by value (```foo(x)```), the *ownership* of the resources is transferred. In Rust-speak, this known as a *move*.

After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers. 

```rust
// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}
```

Reference [./src/demo89.rs](./src/demo89.rs)

#### ***15.2.1 - Mutability***

Mutability of data can be changed when ownership is transferred.

```rust
fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
```

Reference [./src/demo90.rs](./src/demo90.rs)

#### ***15.2.2 - Partial moves***

Within the ```destructuring``` of a single variable, both ```by-move``` and ```by-reference``` pattern bindings can be used at the same time. Doing this will result in a *partial move* of the variable, which means that parts of the variable will be moved while other parts stay. In such a casse, the parents variable cannot be used afterwards as a whole, however the parts that are that are only referenced (and not moved) can still be uesd. 

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
```

Reference [./src/demo91.rs](./src/demo91.rs)

(In this example, we store the ```age``` variable on the heap to illustrate the partial move: deleting ```ref``` in the above code would give an error as the ownership of ```person.age``` would be moved the variable ```age```. If ```Person.age``` were stored on the stack, ```ref``` would not be required as the definition of ```age``` would copy the data from ```person.age``` without moving it.)

### ***15.3 - Borrowing***

Most of the time, we'd like to access data without taking ownership over it. To accomplish this, Rust uses a *borrowing* mechanism. Instead of passing objects by value (```T```), objects can be passed by reference(```&T```). 

The compiler statically guarantees (via its borrow checker) that references *always* point to valit objects. That is, while references to an objects exist, the object cannot be destroyed.

```rust
// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}
```

Reference [./src/demo92.rs](./src/demo92.rs)

#### ***15.3.1 - Mutability***

Mutable data can be mutably borrowed using ```&mut T```. This is called a *mutable reference* and gives read/write access to the borrower. In contrast, ```&T``` borrows the data via an immutable reference, and the borrower can read the data but not modify it: 

```rust
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;
    
    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);
    
    // Borrow a mutable object as mutable
    new_edition(&mut mutabook);
    
    // Error! Cannot borrow an immutable object as mutable
    new_edition(&mut immutabook);
    // FIXME ^ Comment out this line
}
```

Reference [./src/demo93.rs](./src/demo93.rs)

#### ***15.3.2 - Aliasing***

Data can be immutably borrowed any number of times, but while immutably borrowed, the original data can't be mutably borrowed. On the other hand, only *one* mutable borrow is allowed at a time. The original data can be borrowed again only *after* the mutable reference has been used for the last time.

```rust
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;
    // TODO ^ Try uncommenting this line

    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line

    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
```

Reference [./src/demo94.rs](./src/demo94.rs)

#### ***15.3.3 - The ref pattern***

When doing pattern matching or destructuring via the ```let``` binding, the ```ref``` keyword can be used to take references to the fields of a ```struct/tuple```. The example below shows a few instances where this can be useful: 

```rust
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);
}
```

Reference [./src/demo95.rs](./src/demo95.rs)

### ***15.4 - Lifetimes***

A *lifetime* is a construct the compiler (or more specifically, its *borrow checker*) uses to ensure all borrows are valid. Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. While lifetiems and scopes are often referred to together, they are not the same. 

Take, for example, the case where web borrow a variable via ```&```. The borrow has a lifetime that is determined by where it is declared. As a result, the borrow is valid as long as it ends before the lender is destroyed. However, the scope of the borrow is determind by where the reference is used. 

In the following example and in the rest of this section, we will see how lifetimes relate to scopes, as well as how the two differ.

```rust
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

Note that no names or types are assigned to label lifetimes. This restricts how lifetimes will be able to be used as we will see.

#### ***15.4.1 - Explicit annotation***

The borrow checker uses explicit lifetime annotations to determine how long references should be valid. In cases where lifetimes are not elided, Rust requires explicit annotaions to determine what the lifetime of a reference should be. The syntax for explicitly annotating a lifetime uses an apostrophe character as follows: 

```rust
foo<'a>
// `foo` has a lifetime parameter `'a`
```

Similar to ```closures```, using lifetime requires generics. Additionally, this lifetime syntax indicates that the lefetime of ```foo``` may not exceed that of ```'a```. Explicit annotation of a tyupe has the form ```&'a T``` where ```'a``` has already been introduced. 

In cases with multiple lifetimes, the syntax is similar: 

```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```

In this case, the lifetime of ```foo``` cannot exceed that of either ```'a``` or ```'b```. 

See the following example for explicit lifetime annotation in use.

```rust 
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation 
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one.
}

fn main() {
    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);
    
    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower. 
    // In other words, the lifetime of `four` and `nine` must 
    // be longer than that of `print_refs`.
    
    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be 
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
```

Reference [./src/demo96.rs](./src/demo96.rs)

```elision``` implicity annotates lifetime and so is different.

#### ***15.4.2 - Functions***

Ignoring elision, function signatures with lifetimes have a few constraints: 

* Any reference *must* have an annotated lifetime. 
* Any reference being returned *must* have the same lifetime as an input or be ```static```.

Additionally, note that returning references without input is banned if it would result in returning references to invalid data. The following example shows off some valid forms of functions with lifetime: 

```rust
// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

fn main() {
    let x = 7;
    let y = 9;
    
    print_one(&x);
    print_multi(&x, &y);
    
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
```

Reference [./src/demo97.rs](./src/demo97.rs)

#### ***15.4.3 - Methods***

Methods are annotated similarly to functions 

```rust
struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
```

Reference [./src/demom98.rs](./src/demo98.rs)

#### ***15.4.4 - Structs***

Annotation of lifetimes in structures are also similar to functions: 

```rust
// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

Reference [./src/demo99.rs](./src/demo99.rs)

#### ***15.4.5 - Traits***

Annotation of lifetime in strait methods basically are similar to functions. Note that ```impl``` may have annotation of lifetime too. 

```rust
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```

Reference [/src/demo100.rs](./src/demo100.rs)

#### ***15.4.6 - Bounds***

Just like generic types can be bounded, lifetimes (themeselves generic) use bounds as well. The ```:``` character has a slightly different meaning here, but ```+``` is the same. Note how the following read: 

1. ```T: 'a```: *All* references in ```T``` must outlive lifetime ```'a```. 
2. ```T: Trait + 'a```: Type ```T``` must implement trait ```Trait``` and *all* references in ```T``` must outlive ```'a```. 

The example below shows the above syntax in action used after keyword ```where```:

```rust
use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

Reference [./src/demo101.rs](./src/demo101.rs)

#### ***15.4.7 - Coercion***

A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in. This comes in the form of inferred coercion by the Rust compiler. and also in the form of declaring a lifetime difference: 

```rust 
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    
    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

Reference [/src/demo1102.rs](./src/demo102.rs)

#### ***15.4.8 - Static***

Rust has a few reserved lifetime names. One of those is ```'static```. You might encounter it in two situations. 

```rust
// A reference with 'static lifetime:
let s: &'static str = "hello world";

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}
```

Both are related but subtly different and this is a common source for confusion when learning Rust. Here are some example for each situation. 

**Reference lifetime**

As a reference lifetime ```'static``` indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coerced to a shorter lifetime. 

There are two ways to make a variable with ```'static``` lifetime, and both are stored in the read-only memory of the binary: 

* Make a constant with the ```static``` declaration. 
* Make a ```string``` literal which has type: ```&'static str```. 

See the following example for a display of each method

```rust
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```

Reference [./src/demo103.rs](./src/demo103.rs)

**Trait bound**

As a trait bound, it means the type does not contain any non-static reference. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until the drop it. 

It's important to understand this means that any owned data always passes a ```'static``` lifetime bound, but a reference to that owned data generally does not: 

```rust
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
```

The compiler will tell you: 

```shell
error[E0597]: `i` does not live long enough
  --> src/lib.rs:15:15
   |
15 |     print_it(&i);
   |     ---------^^--
   |     |         |
   |     |         borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
16 | }
   | - `i` dropped here while still borrowed
```

#### ***15.4.9 - Elision***

Some lifetime patterns are overhelmingly common and so the borrow checker will allow you to omit them to save typing and to improve readability. This is known as elision. Elision exists in Rust solely because these patterns are common. 

The following code shows a few examples of elision. For a more comprehensive description of elision, see ```lifetime elision``` in the book. 

```rust
// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler:
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
```

Reference [./src/demo104.rs](./src/demo104.rs)

## **16 - Traits**

A ```trait``` is a collection of methods defined for an unknow type: ```Self```. They can access other methods declared in the same trait. 

Traits can be implemented for any data type. In the example below, we define ```Animal```, a group of methods. The ```Animal trait``` is then implemented for the ```Sheep``` data type, allowing the use of methods from ```Animal``` with ```Sheep```. 

```rust
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```

Reference [./src/demo105.rs](./src/demo105.rs)

### ***16.1 - Derive***

The compiler is capable of providing basic implementations for some traits via the ```#[derive]``` **attribute**. These traits can still be manually implemented if a more complex behavior is required. 

The following is a list of derivable traits: 

* Capraison traits: ```Eq```, ```PartialEq```, ```Ord```, ```PartialOrd```.
* ```Clone```, to create ```T``` from ```&T``` via a copy. 
* ```Copy```, to give a type 'copy semanitics' instead of 'move semantics'. 
* ```Hash```, to compute a hash from ```&T```. 
* ```Default```, to create an empty instance of a data type. 
* ```Debug```, to format a value using the ```{:?}``` formater.

```rust
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
```

Reference [./src/demo106.rs](./src/demo106.rs)

### ***16.2 - Returning Traits with dyn***

The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a conrete type. Unlike other languages, if you have a trait like ```Animal```, you can't write a function that returns ```Animal```, because its different implementations will need differenent amounts of memory.

However, there's an easy workaround. Instead of returning a trait object directly, out functions return a ```Box``` which *contains* some ```Animal```. A ```box``` is just a reference to some memory in the heap. Because a reference has a statically-known size, and the compiler can guarantee it points to a heap-allocated ```Animal```, we can return a trait from our function!

Rust tries to be as explicit as possible whenever it allowcates memory on the heap. So if your function returns a pointer-to-traint-on-heap in this way, you need to write the return type with the ```dyn``` keyword, e.g. ```Box<dyn Animal>```.

```rust
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```

Reference [./src/demo107.rs](./src/demo107.rs)

### ***16.3 - Operator Overloading***

In Rust, many of the operators can be overloaded via traits. That is, some operators can be used to accomplish different tasks basee on their input arguments. This is possible because operators are syntactic sugar for method calls. For example, the ```+``` operator in ```a + b``` calls the ```add``` method (as in ```a.add(b)```). This ```add``` method is part of the ```Add``` trait. Hence, the ```+``` operator can ben used by any implementor of the ```Add``` trait. 

A list of the traits, such as ```Add```, that overload operators can be found in ```core::ops```.

```rust
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
```

Reference [./src/demo109.rs](./src/demo108.rs)

### ***16.4 - Drop***

The ```Drop``` trait only has one method: ```drop```, which is called automatically when an object goes out of scope. The main use of the ```Drop``` trait is to free the resources that the implementor instance owns. 

```Box```, ```Vec```, ```String```, ```File```, and ```Process``` are some examples of types that implement the ```Drop``` trait to free resource. The ```Drop``` trait can also be manually implementated for any custom data type. 

The following example adds a print to console to the ```drop``` function to announce when it is called.

```rust
struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}
```

Reference [./src/demo109.rs](./src/demo109.rs)

### ***16.5 - Iterators***

The ```Iterator``` trait is used to implement iterator over collections such as arrays. 

The trait requires only a method to be defined for the ```next``` element, which may be manually defined in an ```impl``` block or automatically defined (as in arrays and ranges). 

As a point of convenience for common situations, the ```for``` construct turns some collections into iterators using ```.into_iter()``` method

```rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
```

Reference [./src/demo110.rs](./src/demo110.rs)

### ***16.6 - impl Trait***

```impl Trait``` can be used in two locations: 

1. As an argument type 
2. As a return type

**As an argument type**

If your information is generic over a trait but you don't mind the specific type, you can simplify the function declaration using ```impl Trait``` as the type of the argument.

For example, consider the following code: 

```rust
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}
```

```parse_csv_document``` is generic, allowing it to take any type which implements BufRead, such as ```BufReader<File>``` or ```[u8]```, but it's not important what type ```R``` is, and ```R``` is only used to declare the type of ```src```, so the function can also be written as:

```rust
fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}
```

Note that using ```impl Trait``` as an argument type means that you cannot explicitly state what form of the function you use, i.e. ```parse_csv_document::<std::io::Empty>(std::io::empty())``` will not work with the second example.

**As a return type**

If your function returns a type that implements ```MyTrait```, you can write its returns type as ```-> impl MyTrait```. This can help simplify your type signatures quite a lot!

```rust
use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}
```

Reference [./src/demo111.rs](./src/demo111.rs)

More importantly, some Rust types can't be written out. For example, every clousure has its own unnamed concrete type. Before ```impl Trait``` syntax, you had to allocate on the heap in order to return a closure. But now you can do it all statically, like this: 

```rust
// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

You can also use ```impl Trait``` to return an iterator that use ```map``` or ```filter``` closures! This makes using ```map``` and ```filter``` easier. Because closure types don't have names, you can't write out an explicit return type if your function returns iterators with closures. But with ```impl Trait``` you can do this easily.

```rust
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
```

### ***16.7 - Clone***

When dealing with resources, the default behavior is to transfer them during assignments or function calls. However, sometimes we need to make a copy of the resource as well. 

The ```Clone``` trait helps us do exactly this. Most commonly, we can use the ```.clone()``` method defined by the ```Clone``` trait.

```rust
// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```

Reference [./src/demo112.rs](./src/demo112.rs)

### ***16.8 - Supertraits***

Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait. For example:

```rust
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}

```

### ***16.9 - Disambiguating overlapping traits***

A type can implement many different traits. What if two traits both require the same name? For example, many traits might have a method named ```get()```. They might even have different return types!

Good news: because each trait implementation gets its own ```impl``` block, it's clear which traint's ```get``` method you're implementing.

What about when it comes time to *call* those methods? To disambiguate between them, we have to use Fully Qualified Syntax.

```rust
trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
```

Reference [./src/demo113.rs](./src/demo113.rs)

## **17 - Macros_RusMaless **

Rust provides a powerful macro system that allows metaprogramming. As you've see in previous chapter, macros look like functions, except that their name ends with a bang ```!```, but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs. 

Macros are created using the ```macro_rules!``` macro. 

```rust
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
```

Reference [./src/demo114.rs](./src/demo114.rs)

So why are macros useful?

1. Don't repeat yourself. There are many cases where you may need similar functionality in multiple places but with different types. Often, writing a macro is a useful way to avoid repeating code. (More on this later)

2. Domain-specific languages. Macros allow you to define special syntax for a specific purpose. (More on this later)

3. Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is ```println!``` which could take any number of argument, depending on the format string!. (More on this later)

### ***17.1 - Syntax***

In following subsections, we will show how to define macros in Rust. There are three basic ideas:

* Patterns and Designators
* Overloading
* Repetition

#### ***17.1.1 - Designators***

The arguments of a macro are prefixed by dollar sign ```$``` and type annotated with a *designator*:

```rust
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
```

Reference [./src/demo115.rs](./src/demo115.rs)

These are some of the variable designators: 

* ```block```
* ```expr``` is used for expressions
* ```ident``` is used for variable/function names
* ```item```
* ```literal``` is used for literal constants
* ```pat``` (pattern)
* ```path```
* ```stmt``` (statement)
* ```tt``` (token tree)
* ```ty``` (type)
* ```vis``` (visibility qualifier)

For a complete list, see the [Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html)

#### ***17.1.2 - Overload***

Macros can be overloaded to accept different combinations of arguments. In that regard, ```macro_rules!``` can work simiplarly to a match block:

```rust
// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```

Reference [./src/demo116.rs](./src/demo116.rs)

#### ***17.1.3 - Repeat***

Macros can use ```+``` in the argument list to indicate that an argument may repeat at least once, or ```+```, to indicate that the argument may repeat zero or more times.

In the following example, surrounding the matcher with ```$(...),+``` will match one or more expression, separated by commas, Also note that the semicolon is optional on the last case.

```rust
// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
```

Reference [./src/demo117.rs](./src/demo117.rs)

### ***17.2 - DRY (Don't Repeat Yourself)***

Macros allow writting DRY code by factoring out the common parts of functions and/or test suites. Here is an example that implements and tests the ```+=```, ```*=``` and ```-=``` operators on ```Vec<T>```:

```rust
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    // Test `add_assign`, `mul_assign`, and `sub_assign`.
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
```

```shell
$ rustc --test dry.rs && ./dry
running 3 tests
test test::mul_assign ... ok
test test::add_assign ... ok
test test::sub_assign ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

### ***17.3 - DSL (Domain Specific Languages)***

A DSL is a mini "language" embedded in a Rust macro. It is completely valid Rust because the macro system expands into normal Rust constructs, but it looks like a small language. This allows you to define concise or intuitive syntax for some special functionality (within bounds). 

Suppose that I want to define a little calculator API. I would like to supply an expression and have the output printed to console.

```rust
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
```

Reference [./src/demo118.rs](./src/demo118.rs)

Output:

```shell
1 + 2 = 3
(1 + 2) * (3 / 4) = 0
```

This was a very simple example, but much more complex interfaces have been developed, such as ```lazy_static``` or ```clap```.

Also, note the two pairs of braces in the macro. The outer ones are part of the syntax of ```macro_rules!```, in addition to ```()``` or ```[]```.

### ***17.4 - Variadics***

A *variadic* interface takes an arbitrary number of arguments. For example, ```println!``` can take an arbitrary number of arguments, as determined by the format string.

We can extend our ```calculate!``` macro from the previous section to be variadic:

```rust
macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

Reference [./src/demo119.rs](./src/demo119.rs)

Output:

```shell
1 + 2 = 3
3 + 4 = 7
(2 * 3) + 1 = 7
```

## **18 - Error handling**

Error handling is the process of handling the possibility of failure. For example, failing to read a file and then continuing to use that *bad* input would clearly be problemmatic. Noticing and explicitly managing those errors saves the rest of the program from various pitfalls.

There are various ways to deal with errors in Rust, which are described in the following subchapters. They all have more or less subtle different use cases. As a rule of thumb: 

An explicit ```panic``` is mainly useful for tests and dealing with unrecoverable errors. For prototyping it can be useful, for example when dealing with functions that haven't been implemented yet, but in those cases the more descriptive ```unimplemented``` is better. In tests ```panic``` is a reasonable way to explicitly fail.

The ```Option``` type is for when a value is opotional or when the lack of a value is not an error condition. For example the parent of a directory ```-``` ```/``` and ```c:```don't have one. When dealing with ```Option``` S, ```unwrap``` is fine for prototyping and cases where it's absolutely certain that there is guaranteed to be value. However ```epect``` is more useful since it lets your specify an error message in case something goes wrong anyway. 

When there is a chance that things do go wrong and the caller has to deal with the problem, use ```Result```. You can ```unwrap``` and ```expect``` them as well (please don't do that unless it's a test or quick prototype). 

For a more rigorous discusstion of error handling, refer to the error handling section in the [official book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

### ***18.1 - Panic***

The simplest error handling mechanism we will see is ```painc```. It prints an error message, starts unwinding the stack, and usually exits the program. Here, we explicitly call ```panic``` on out error condition: 

```rust
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

Reference [./src/demo120.rs](./src/demo120.rs)

Output

```shell
 Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/playground`
thread 'main' panicked at 'AAAaaaaa!!!!', src/main.rs:3:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Some refreshing water is all I need.
```

### ***18.2 - Abort & Unwind***

The previous section illustrates the error handling mechanism ```panic```. Different code paths can be conditionally compiled based on the panic setting. The current values available are ```unwind``` and ```abort```.

Building on the prior lemonade example, we explicaitly use the panic strategy to exercise different lines of code.

```rust

fn drink(beverage: &str) {
   // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic="abort"){ println!("This is not your party. Run!!!!");}
        else{ println!("Spit it out!!!!");}
    }
    else{ println!("Some refreshing {} is all I need.", beverage); }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

Reference [./src/demo121.rs](./src/demo121.rs)

Output

```shell
Some refreshing water is all I need.
Spit it out!!!!
```

Here is another example focusing on rewriting ```drink()``` and explicitly use the ```unwind``` keyword. 

```rust

#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party. Run!!!!");}

fn drink(beverage: &str){
    if beverage == "lemonade"{ ah();}
    else{println!("Some refreshing {} is all I need.", beverage);}
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

Reference [./src/demo122.rs](./src/demo122.rs)

Output 

```shell
Some refreshing water is all I need.
Spit it out!!!!
```

The panic strategy can be set from the command line by using ```abort``` or ```unwind```

```shell
rustc lemonade.rs -C panic=abort
```

### ***18.3 - Option & Unwrap***

In the last example, we showed that we can induce program failure at will. We told our program to ```panic``` if we drink a sugary lemonade. But what if we expect *some* drink but don't receive one? That case would be just as bad, so it needs to be handled!

We could *test* this against the null string (```""```) as we do with a lemonade. Since we're using Rust, let's instead have the compiler point out cases where there's no drink. 

An ```enum``` called ```Option<T>``` in the ```std``` library is used when absence is a possibility. It manifasts itself as one of two "options": 

* ```Some(T)```: an element of type ```T``` was found
* ```None```: no element was found.

These cases can either be explicitly handled via ```match``` or implicitly with ```unwrap```. Implicit handling will either return the inner element or ```panic```. 

Note that it's possible to manuually customize ```panic``` with ```expect```, but ```unwrap``` otherwise leaves us with a less meaningful output than expicit handling. In the following example, explicit handling yields a more controlled result while retaining the option to ```panic``` if desired. 

```rust
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
```

Reference [./src/demo123.rs](./src/demo123.rs)

#### ***18.3.1 - Unpacking options with ?***

You can unpack ```Option```S by using ```match``` statements, but it's often easier to use the ```?``` operator. If ```x``` is an ```Option```, then evaluating ```x?``` will return the underlying value if ```x``` is ```Some```, otherwise it will terminate whatever function is being executed and return ```Note```. 

```rust
fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}
```

You can chain many ```?``` s together to make your code much more readable.

```rust
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```

Reference [./src/demo124.rs](./src/demo124.rs)

#### ***18.3.2 - Combinators: map***

```match``` is a valid method for handling ```Option```S . However, you many eventually find heavy usage tedious, especially with operations only valid with an input. In these cases, ```combinators``` can be used to manage control flow in a modular fashion.

```Option``` has a built in method called ```map()```, a combinator for the simple mapping of ```Some -> Some``` and ```None -> None```. Multiple ```map()``` calls can be chained together for even more flexibility. 

In the following example, ```process()``` replaces all functions previous to it while staying compact.

```rust
#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// Otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// Otherwise, return the chopped food.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
```

Reference [./src/demo125.rs](./src/demo125.rs)

Ouput

```shell
Mmm. I love Cooked(Apple)
Mmm. I love Cooked(Carrot)
Oh no! It wasn't edible.
```

#### ***18.3.3 - Combinators: and_then***

```map()``` was described as a chainable way to simplify ```match``` statements. However, using ```map()``` on a function that returns an ```Option<T>``` results in the nested ```Option<Option<T>>```. Chaining multiple calls together can then become confusing. That's where another combinator called ```and_then()```, know in some languages as flatmap, comes in. 

```and_then()``` calls its function input with the wrapped value and returns the result. If the ```Option``` is ```None```, then it returns ```None``` instead.

In the following example, ```cookable_v2()``` results in an ```Option<Food>```. Using ```map()``` instead of ```and_then()``` would have given an ```Option<Option<Food>>``, which is an invalid type for ```eat()```.

```rust
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => match have_ingredients(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
```

Reference [./src/demo126.rs](./src/demo126.rs)

#### ***18.3.4 - Defaults: or, or_else, get_or_insert, 'get_or_insert_with'***

The is more than one way to unpack an ```Option``` and fall back on a default if it is ```None```. To choose the one that meets our needs, we need to consider the following: 

* Do we need eager or lazy evaluation?
* Do we need to keep the original empty value intact, or modify it in place?

**```or()``` is chainable, evaluates eagerly, keeps empty value intact**

```or()``` is chainable and eagerly evaluates its argument, as is shown in the following example. Note that because ```or``` 's arguments are evaluated eagerly, the variable passed to ```or``` is moved.

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` moves its argument.
    // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
    // But the variable named `apple` has been moved regardless, and cannot be used anymore.
    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error
 }
```

```shell
first_available_fruit: Some(Orange)
```

**```or_else()``` is chainable, evaluates lazily, keeps empty value intact**

Another alternative is to use ```or_else```, which is also chainable, and evaluates lazily, as is shown in the following example:

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // Providing kiwi as fallback
    // first_available_fruit: Some(Kiwi)
}
```

```shell
Providing kiwi as fallback
first_available_fruit: Some(Kiwi)
```

**```get_or_insert()``` evaluates eagerly, modifies empty value in place**

To make sure that an ```Option``` cotains a value, we can use ```get_or_insert``` to modify it in place with a fallback value, as is shown in the following example. Note that ```get_or_insert``` eagerly evaluaes its parameter, so variable ```apple``` is moved: 

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // my_fruit is: Apple
    // first_available_fruit is: Apple
    //println!("Variable named `apple` is moved: {:?}", apple);
    // TODO: uncomment the line above to see the compliler error
}
```

```shell
my_fruit is: Apple
first_available_fruit is: Apple
```

**```get_or_insert_with()``` evaluates lazily, modifies empty value in place**

Instead of explicitly providing a value to fall back on, we can pass a closure to ```get_or_insert_with```, as follows: 

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // Providing lemon as fallback
    // my_fruit is: Lemon
    // first_available_fruit is: Lemon

    // If the Option has a value, it is left unchanged, and the closure is not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
    // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
}
```

```shell
Providing lemon as fallback
my_fruit is: Lemon
first_available_fruit is: Lemon
should_be_apple is: Apple
my_apple is unchanged: Some(Apple)
```

### ***18.4 - Result***

```Result``` is a richer version of the ```Option``` type that describles possible *error* instead of possible *obsence*.

That is, ```Result<T, E>``` could have one of two outcomes: 

* ```Ok(T)```: an element ```T``` was found
* ```Errr(E)```: an error was found with element ```E```

By convention, the expected outcom is ```Ok``` while the unexpected outcome is ```Err```.

Like ```Option```, ```Result``` has many methods associated with it. ```unwrap()```, for example, either yields the element ```T``` or ```painc```S. For case handling, there are many combinators between ```Result``` and ```Option``` that overlap. 

In working with Rust, you will likely encounter methods that return the ```Result``` type, such as the ```parse()``` method. It might not always be possible to parse as string into the other type, so ```parse()``` returns a ```Result``` indicating possible failure. 

Let's seee what happens when we successfully and unsuccessfully ```parse()``` a string:

```rust
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
```

Reference [./src/demo127.rs](./src/demo127.rs)

In the unsuccessful case, ```parse()``` leaves us with an error for ```unwrap()``` to ```panic``` on. Additionally, the ```panic``` exits our program and provides an unpleasant error message. 

To improve the quality of our error message, we should be more specific about the return type and consider explicitly handling the error. 

**Using Result in main**

The ```Result``` type can also be the return type of the ```main``` function if specified explicitly. Typically the ```main``` function will be of the form: 

```rust
fn main() {
    println!("Hello World!");
}
```

However ```main``` is also able to have a return type of ```Result```. If an error occurs withing the ```main``` function it will return an error code and print a debug representation of the error (using the ```Debug``` trait). The following example shows such a scenario and touches on aspects covered in *the following section*

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

```shell
10
```

#### ***18.4.1 - map for Result***

Panicking in the previous example's ```multiply``` does't make for robust code. Generally, we want to return the error to the caller so it can decide what is the right way to respond to errors. 

We first need to know what kind of error type we are dealing with. To determine the ```Err``` type, we look to ```parse()```, which is implemented with the ```FromStr``` trait for ```i32```. As a result, the ```Err``` type is specified as ```ParseIntError```.

In the example below, the straightforward ```match``` statement leads to code that is overall more cumbersome.

```rust
use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);
}
```

Reference [./src/demo128.rs](./src/demo128.rs)

Luckily, ```Option``` 'S ```map```, ```and_then```, and many other combinators are also implemented for ```Result```. ```Result``` contains a complete listing. 

```rust
use std::num::ParseIntError;

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);
}
```

Reference [./src/demo129.rs](./src/demo129.rs)

#### ***18.4.2 - aliases for Result***

How about when we want to reuse a specific ```Result``` type many times? Recall that Rust allows us to create ```alias```. Conveniently, we can define one for the specific ```Result``` in question. 

At a module level, creating aliases can be particularly helpful. Errors found in a specific module often have the same ```Err``` type, so a single alias can succincty define *all* associated ```Result```. This is so useful that the ```std``` library even aupplies one: ```io::Result!```

Here's a quick example to show off the syntax

```rust
use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

Reference [./src/demo130.rs](./src/demo130.rs)

#### ***18.4.3 - Early returns***

In the previous example, we explicitly the errors using combinators. Another way to deal with this case analysis is to use a combination of ```match``` statements and *early returns*. 

That is, we can simply stop executing the function and return the error if one occurs. For some, this form of code can be easier to both read and write. Consider this version of the previous example, rewritten using early returns: 

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

Reference [./src/demo131.rs](./src/demo131.rs)

At this point, we've learned to explicitly handle errors using combinators and early returns. While we generally want to avoid panicking, explicitly handling all of our errors is cumbersome. 

In the next section, we'll introduct ```?``` for the cases where we simply need to ```unwrap``` without possibly inducing ```panic```.

#### ***18.4.4 - Introducing?***

Sometiems we just want the simplicitly of ```unwrap``` without the possibility of a ```panic```. Until now, ```unwrap``` has forced us to nest deeper and deeper when what really wanted was to get the variable *out*. This is exactly the purpose of ```?```.

Upon finding an ```Err```, there are two valid actions to take: 

1. ```panic!``` which we already decided to try to avoid if possible
2. ```return``` because an ```Err``` means it cannot be handled

```?``` is *almost* exactly equivalent to an ```unwrap``` which ```return``` S instead of ```panic``` king on ```Err``` S. Let's see how we can simplify the earlier example that used combinators:

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

Reference [./src/demo132.rs](./src/demo132.rs)

```shell
n is 20
Error: invalid digit found in string
```

**The try! macro**

Before there was ```?```, the same functionality was achieved with the ```try!```. The ```?``` operator is now recommended, but you may still find ```try!``` when looking at older code. The same ```multiply``` function from the previous example would look like this using ```try!```: 

```rust
// To compile and run this example without errors, while using Cargo, change the value 
// of the `edition` field, in the `[package]` section of the `Cargo.toml` file, to "2015".

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

Reference [./src/demo133.rs](./src/demo133.rs)

### ***18.5 - Multiple error types***

The previous examples have always been very convenient; ```Result```S interact with other ```Result```S and ```Option```S interact with other ```Option```S. 

Sometimes an ```Option``` needs to interact with a ```Result```, or a ````Result<T, Error1>``` needs to interact with a ```Result<T, Error2>```. In those cases, we want to manage our different error types in a way that makes them composable and easy to interact with. 

In the following code, two instances of ```unwrap``` generate different error types. ```Vec::first``` returns an ```Option```, while ```parse::<i32>``` returns a ```Result<i32, ParseIntError>```:

```rust
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}
```

Refernece [./src/demo134.rs](./src/demo134.rs)

```shell
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/playground`
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:2:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
The first doubled is 84
```

Over the next sections, we'll see serveral strategies for handling these kind of problems.

#### ***18.5.1 - Pulling Results out of Options***

The most basic way of handling mixed error types is to just embed them in each other.

```rust
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}
```

```shell
The first doubled is Some(Ok(84))
The first doubled is None
The first doubled is Some(Err(ParseIntError { kind: InvalidDigit }))
```

There are times when we'll want to stop processing on errors (like with ```?```) but keep going when the ```Option``` is ```None```. A couple of combinators come in handy to swap the ```Result``` and ```Option```. 

```rust
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
```

```shell
The first doubled is Ok(Some(84))
The first doubled is Ok(None)
The first doubled is Err(ParseIntError { kind: InvalidDigit })
```

#### ***18.5.2 - Defining an error type***

Sometimes it simplifies the code to mask all of the different errors with a single type of error. We'll show this with a custom error. 

Rust allows us to define our own error types. In general, a "good" error type:

* Repensents different errors with the same type
* Presents nice error message to the user 
* Is easy to compare with other types
    * Good: ```Err(EmptyVec)```
    * Bad: ```Err("Please use a vector with at least one element:.to_owned())```
* Can hold information about the error
    * Good: ```Err(BadChar(c, position))```
    * Bad: ```Err("+ cannot be used here".to_owned())```
* Composes well with other errors

```rust
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct DoubleError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Change the error to our new type.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to the new error type here also.
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

Reference [./src/demo135.rs](./src/demo135.rs)

```shell
The first doubled is 84
Error: invalid first item to double
Error: invalid first item to double
```

#### ***18.5.3 - Boxing errors***

A way to write simple code while preserving the original errors is to ```Box``` them. The drawback is that the underlying error type is only known at runtime and not ```statically determined```.

The stdlib helps in boxing our errors by having ```Box``` implement conversion from any type that implements the ```Error``` trait the trait object ```Box<Error>```. via ```From```.

```rust
use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

Reference [./src/demo136.rs](./src/demo136.rs)

Output

```shell
The first doubled is 84
Error: invalid first item to double
Error: invalid digit found in string
```

#### ***18.5.4 - Other uses of?***

Notice in the previous example that our immediate reaction to calling ```parse``` is to ```map``` the error from a library error into a boxed error: 

```rust
.and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())
```

Since this is a simple and common operation, it would be convenient if it could elided. Alas, because ```and_then``` is not sufficiently flexible, it cannot. However, we can instead use ```?```.

```?``` was previously explained as either ```unwrap``` or ```return Err(err)```. This is only mostly true, it actually means ```unwrap``` or ```return Err(From::from(err))```. Since ```From::from``` is a conversion utility between different types, this means that if you ```?``` where the error is convertible to the return type, ti will convert automatically. 

Here, we rewrite the previous example using ```?```. As a result, the ```map_err``` will go away when ```From::from``` is implemented for our error type: 

```rust
use std::error;
use std::fmt;

// Change the alias to `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

Reference [./src/demo137.rs](./src/demo137.rs)

Output

```shell
The first doubled is 84
Error: invalid first item to double
Error: invalid digit found in strin
```

This is actually fairly clean now. Compared with the original ```panic```, it is very similar to replacing the ```unwrap``` call with ```?``` except that the return types are ```Result```. As a result, they must be destructured at the top level.

#### ***18.5.5 - Wrapping errors***

An alternative to boxing errors is to wrap them in your own error type

```rust
use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From` (which
    // we defined above) in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

Reference [./src/demo138.rs](./src/demo138.rs)

Output 

```shell
The first doubled is 84
Error: please use a vector with at least one element
Error: the provided string could not be parsed as int
  Caused by: invalid digit found in string
```

This adds a bit more boilerplate for handling errors and might not be needed in all applications. There are some libraries that can take care of the boilerplate for you.

### ***18.6 - Iterating over Results***

An ```Iter::map``` operation might fail, for example

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

```shell
Results: [Err(ParseIntError { kind: InvalidDigit }), Ok(93), Ok(18)]
```

Let's step through strategies for handling this. 

**Ignore the failed items with ```filter_map()```**

```filter_map()``` calls a function and filter out the result that are ```None```.

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}
```

```shell
Results: [93, 18]
```

**Collect the failed items with ```map_err()``` and ```filter_map()```**

```map_err``` calls a function with the error, so by adding that to the previous ```filter_map``` solution we can save them off to the side while iterating.

```rust
fn main() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

```shell
Numbers: [42, 93, 18]
Errors: [ParseIntError { kind: InvalidDigit }, ParseIntError { kind: PosOverflow }]
```

**Fail the entire operation with ```collect()```** 

```Result``` implements ```FromtIter``` so that a vector of results (```Vec<Result<T, E>>``` ) can be turned into a result with a vector (```Result<Vec<T>, E>```). Once an ```Result::Err``` is found, the iteration will terminate.

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

```shell
Results: Err(ParseIntError { kind: InvalidDigit })
```

This same technique can be used with ```Option```


**Collect all valid values and failures with ```partition()```**

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

```shell
Numbers: [Ok(93), Ok(18)]
Errors: [Err(ParseIntError { kind: InvalidDigit })]
```

When you look at the results, you'll note that everyhing is still wrapped in ```Result```. A little more boierplate is needed for this.

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

```shell
Numbers: [93, 18]
Errors: [ParseIntError { kind: InvalidDigit }]
```

## **19 - Std library types**

## **20 - Std misc**

## **21 - Testing**

## **22 - Unsafe Operations**
## **23 - Compatibility**

## **24 - Meta**