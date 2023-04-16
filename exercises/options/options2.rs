// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // Explanation: if let does pattern matching, we can use `Some(word)`
        //to extract the value inside `optional_target`
        if let Some(word) = optional_target {
            //pattern matching
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(Some(integer)) = optional_integers.pop() {
            // Explanation: the outer `Some` is for pattern matching with `pop()`, the inner
            //`Some` is for pattern matching with Vec<Option> (Doing value extraction two times).
            // `While let` is used to accomondate `.pop()`. This means while the value that `.pop()` returns
            //can match the pattern `Some(Some(integer))`, we enter into the while code block.
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
