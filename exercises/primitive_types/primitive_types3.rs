// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let a = [1; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array! Fill with {}", a[0]);
    } else {
        println!("Meh, I eat arrays like that for breakfast. Fill with {}", a[0]);
    }
}
