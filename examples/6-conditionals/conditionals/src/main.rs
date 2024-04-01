// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    // let maybe_number: Option<Option<()>> = None;
    let maybe_number = Some(Some(7654));
    if let Some(Some(number)) = maybe_number {
        println!("The number is {:?}", number);
    } else if let Some(Some(42)) = maybe_number {
        println!("The number is {:?}", 42);
    } else {
        println!("There is no number");
    }
}
// Added the else if let block as a way to show that we can have multiple conditions. Was part of the challenge questions