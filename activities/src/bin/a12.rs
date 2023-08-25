// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
  BROWN,
  WHITE,
  BLACK
}

struct Dimension {
  height: i32,
  width: i32,
  depth: i32,
}
struct ShippingBox {
  dimensions: Dimension,
  weight: i32,
  color: Color,
}

impl ShippingBox {
  fn new(dimensions: Dimension, weight: i32, color: Color) -> Self {
    Self { dimensions,  weight, color }
  }

  fn print_characteristics(&self) {
    println!("Heigh: {:?}", self.dimensions.height);
    println!("Width: {:?}", self.dimensions.width);
    println!("Depth: {:?}", self.dimensions.depth);
    println!("Weight: {:?}", self.weight);
    
    match self.color {
      Color::BROWN => println!("Color: Brown"),
      Color::WHITE => println!("Color: White"),
      Color::BLACK => println!("Color: Black"),
    }
  }
}
fn main() {
  let dimension = Dimension {
    height: 180,
    width: 50,
    depth: 10,
  };

  let small_box = ShippingBox::new(dimension, 10, Color::BLACK);
  small_box.print_characteristics();
}
