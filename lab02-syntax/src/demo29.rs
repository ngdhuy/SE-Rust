fn main() {
  let x = 5u32;

  let y = {
      let x_squared = x * x;
      let x_cube = x_squared * x;

      // This expression will be assigned to `y`
      x_cube + x_squared + x
      // don't have ; end of last-line because the value will be assigned to y
  };

  let z = {
      // The semicolon suppresses this expression and `()` is assigned to `z`
      2 * x; 
      // cannot assign value for z because have ; end of last-line
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}
