// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinate() -> (i32, i32) {
  return (10, 10);
}

fn main() {
  let (_x, y) = coordinate();

  if y > 5 {
    println!("{:?} greater than 5", y)
  } else if y == 5 {
    println!("{:?} equal to 5", y)
  } else {
    println!("{:?} less than 5", y)
  }
}
