// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
    // this returns a string slice with 'static lifetime
    // the data of the string lives in the code itself, so if will live as long as the program
    // Create a String out of it
    // String::from("blue") is an alternative
}
