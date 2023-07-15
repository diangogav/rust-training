// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
  let mut count = 0;

  loop {
      count = count + 1;
      if count > 4 {
          break
      }

      println!("{:?}", count);
  }
}
