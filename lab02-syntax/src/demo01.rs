fn main() {
  // general
  println!("{} days", 31);

  // print value with arguments
  println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

  // print with named arguments
  println!("{subject} - {verb}, {object}", 
            object="the lazy dog", 
            subject="the quick brown fox", 
            verb="jumps over");

  // print with specifying the format character
  println!("Base 10:                 {}", 69420);
  println!("Base 2 (binary):         {:b}", 69420);
  println!("Base 8 (octal):          {:o}", 69420);
  println!("Base 16 (hexadecimal):   {:x}", 69420);
  println!("Base 16 (hexadecimal):   {:X}", 69420);

  // format right-justify the text
  println!("{number:>5}", number=123);
  
  // format left-justify the text
  println!("{number:<5}", number=123);

  // format numbers with extra zeroes and left-adjust by flipping the sign
  // this will output "10000"
  println!("{number:0<5}", number=1);

  // and fill 0 before the number -> "00001"
  println!("{number:0>5}", number=1);

  // use named of arguments for format
  println!("{number:0>width$}", number=123, width=6);

  // Only types that implement fmt::Display can be formatted with `{}`
  // User defined types do not implement fmt::Display by default
  #[allow(dead_code)]
  struct Struct(i32);

  let number: f64 = 1.0; 
  let width: usize = 5; 

  println!("{number:>width$}");
}