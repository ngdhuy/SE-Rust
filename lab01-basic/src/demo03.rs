/**
 * * Data types
 */
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
  return type_name::<T>();
}

fn print_array(arr: & [i32]) {
  let mut i = 0;
  while i < arr.len() {
    print!("{}\t", arr[i]);
    i += 1; 
  }
  println!();
}

fn main() {
  // -- Floating-point
  let x = 2.0; // f64
  println!("x is {x} with data-type is {}", type_of(x));

  let y: f32 = 3.0; // f32
  println!("y is {y} with data-type is {}", type_of(y));

  // Numeric operations 
  // addition
  let sum = 5 + 10; 
  println!("sum is {sum} with data-type is {}", type_of(sum));

  // subtraction
  let difference = 95.5 - 4.3; 
  println!("difference is {difference} with data-type is {}", type_of(difference));

  // multiplication
  let product = 4 * 30; 
  println!("product is {product} with data-type is {}", type_of(product));

  // remainder
  let remainder = 43 % 5;
  println!("remainder is {remainder} with data-type is {}", type_of(remainder));

  // -- Boolean type
  let t = true; 
  println!("t is {t} with data-type is {}", type_of(t));

  let f: bool = false;
  println!("f is {f} with data-type is {}", type_of(f));

  // -- Character
  let c = 'z';
  println!("c is {c} with data-type is {}", type_of(c));

  let z: char = 'Z';
  println!("z is {z} with data-type is {}", type_of(z));

  let heart_eyed_cat = '🙀'; 
  println!("heart_eyed_cat is {heart_eyed_cat} with data-type is {}", type_of(heart_eyed_cat));

  // -- Compound types
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (k, m, l) = tup;

  println!("k is {k} with data-type is {}", type_of(k));
  println!("m is {m} with data-type is {}", type_of(m));
  println!("l is {l} with data-type is {}", type_of(l));

  // access element in tuple with index
  println!("First item of tuple at index(0) {}", tup.0);
  println!("Second item of tuple at index(1) {}", tup.1);
  println!("Third item of tuple at index(2) {}", tup.2);

  // -- Array
  let ar = [1, 2, 3, 4, 5];
  print_array(& ar);

  let _num = ["one", "two", "three", "four", "five"];

  // define data-type and number of item for array 
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  // access item/element with index
  let first = arr[0]; 
  println!("first is {first} with data-type is {}", type_of(first));

  let second = arr[1];
  println!("second is {second} with data-type is {}", type_of(second));
}