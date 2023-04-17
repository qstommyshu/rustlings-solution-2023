// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    // Explanation: assert make sure things inside evaluates to true
    // pass
    #[test]
    fn you_can_assert() {
        let result = 2 + 2;
        assert!(result == 4);
    }
    // fail
    // #[test]
    // fn you_can_assert2() {
    //     let result = 2 + 2;
    //     assert!(result == 5);
    // }
}
