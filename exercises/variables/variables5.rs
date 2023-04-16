// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

//Uses shadowing, shadowing is just variables can use the same name, but they still need to be declared.
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    {
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}
