// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

fn main() {
    // Explanation: "milk" is a string slice, so the generic type name should be '&str'
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
