mod data;
use data::{Data, Direction};



fn main() {
    // Create an instance of out Data structure and populate it with data
    // for later use.
    let new_data = Data{name:"toure".to_string(), age:42};

    // Here we reference the name and age fields in our instance of the Data with
    // the dot notation
    println!("Hello, {}", new_data.name);
    println!("Your favorite number is {}", new_data.age);

   // initialize and access enum variants
    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;

    // print enum values with make use of the debug marker ":?" as the data type does
    // not implement the display trait (traits are covered later)
    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);
}
