// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.

// Explanation: we need to pass in self as mutable parameter as we need to append "Bar" to
//the Vector, and by signature, we need to return a Self object. However, there is a little
//thing that I don't quite understand: why the the `self` in trait can become `mut self` in
//the implementation? Are mut self and self considered the same type? I'm sure &self and self
//are different.
impl AppendBar for Vec<String> {
    // Seems mut self and self are considered the same type, but `mut self` is for function
    //signature. It is mainly for readbility(other devs will know this function will mutate
    //the input parameter).
    //References: https://users.rust-lang.org/t/what-is-different-between-mut-self-and-mut-self/59708/2
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_owned());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
