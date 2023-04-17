// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    // Explanation: Syntax is :assert_eq(actual_value, target_value)
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2 + 2, 4);
    }
}
