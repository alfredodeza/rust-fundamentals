
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
