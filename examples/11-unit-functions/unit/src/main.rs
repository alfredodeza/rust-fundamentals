

fn process_numbers(numbers: &[i32]) {
    // Initialize the sum to zero
    let mut sum = 0;

    // Iterate over the numbers, adding each one to the sum
    for number in numbers {
        sum += number;
    }

    // Print the sum
    println!("The sum of the numbers is: {}", sum);

    // If the sum is even, print a message
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}


fn main() {
    process_numbers(&[1,2,3]);
}
