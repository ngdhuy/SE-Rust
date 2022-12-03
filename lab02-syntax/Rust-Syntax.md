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


## **2 - Primitives**

## **3 - Custom Types**

## **4 - Variable Bindings**

## **5 - Types**

## **6 - Conversion**

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