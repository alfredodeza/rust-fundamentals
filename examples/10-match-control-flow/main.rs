// Match control flow
//
//
fn main() {
    let name = "Hello";

    // use of match expression to pattern match against variable "name"
    match name {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet!"),
        _ => println!("I can't find a greeting, good bye."),
    }

}
