fn modify_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(vec[i] + 1);
    }
    new_vec
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    // this is borrowing the variable my_vec
    let first = &my_vec[0];
    modify_vec(my_vec);
    // but this is using first which was borrowed and yet is now invalid
    println!("{:?}", first);
}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.