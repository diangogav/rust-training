// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum Color {
  RED,
  BLUE,
  BLACK,
  WHITE
}
// * Use a function to print the color name

// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn print_color(color: Color) {
  match color {
    Color::BLACK => println!("black"),
    Color::WHITE => println!("white"),
    Color::RED => println!("red"),
    Color::BLUE => println!("blue"),
  }
}
fn main() {
  print_color(Color::BLACK);
  print_color(Color::RED);
  print_color(Color::BLUE);
  print_color(Color::WHITE);
}
