// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Explanation: There are multiple ways to do this question, the method I use
//is using the property that &str has 'static lifetime(lifetime is usually determined
//at initialization), which means &str can
//live long as the program. Hence it won't expire at result.
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is '{}'", result);
}
