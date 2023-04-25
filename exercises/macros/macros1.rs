// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Explanation: just macro syntax, macro is replace this syntax with code  defined in macro.
    my_macro!();
}
