// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // this is called shadowing and we can reassign another content to a new value with the same name, as long as the type is different
    let number: i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
