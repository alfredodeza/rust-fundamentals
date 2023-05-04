// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    let maybe_number: Option<Option<()>> = None;
    //let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }
}
