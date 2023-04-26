// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

// Explanation: Clippy is interesting, even though all these errors are able to
//pass the compiler check, but not good in terms of readability.
fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
