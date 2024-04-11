// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // x needs to have a value otherwise it will not compile,
    // it will automatically be inferred as an i32
    let x = 1;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
