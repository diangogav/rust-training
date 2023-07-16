// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Flavor {
  HOT,
  SPICY,
  SWEET,
  SOUR
}
// * Use a struct to store drink flavor and fluid ounce information

struct Drink {
  flavor: Flavor,
  fluid: f64
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::HOT => println!("flavor: Hot"),
    Flavor::SOUR => println!("flavor: Sour"),
    Flavor::SPICY => println!("flavor: Spicy"),
    Flavor::SWEET => println!("flavor: Sweet")
  }

  println!("fluid: {:?}", drink.fluid);
}

// * Use a match expression to print the drink flavor

fn main() {
  let lemonade = Drink {
    flavor: Flavor::SOUR,
    fluid: 6.0
  };

  let hot_drink = Drink {
    flavor: Flavor::HOT,
    fluid: 5.0
  };

  let spicy_drink = Drink {
    flavor: Flavor::SPICY,
    fluid: 4.0
  };

  let sweet_drink = Drink {
    flavor: Flavor::SWEET,
    fluid: 3.0
  };

  print_drink(lemonade);
  print_drink(hot_drink);
  print_drink(spicy_drink);
  print_drink(sweet_drink);
}
