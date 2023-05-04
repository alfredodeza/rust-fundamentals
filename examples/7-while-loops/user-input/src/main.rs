use std::io;

// This example is a useful application of `while` because it allows to continue
// asking for user input until the user types a specific word (in this case,
// "stop").
fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Goodbye!");
}
