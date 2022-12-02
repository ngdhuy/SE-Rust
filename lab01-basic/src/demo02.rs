/**
 * * Define Variable in RUST
 */

fn main() {
  let x = 5;      // like constance variable - but cannot change value - immutable variable
  println!("The value of x is: {x}");
  
  // mutable variable
  let mut y = 6;  // define variable - can change value
  println!("The value of y is: {y}");

  y = 100; 
  println!("The value of y is: {y}");

  // constants - const
  const PI : f32 = 3.14;    // float with 32 bit
  println!("PI = {PI}");

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

  // variable cannot change data-type if we using mut keyword
  let value = "abc def"; 
  let value = value.len(); 
  println!("Value is: {value}");
}