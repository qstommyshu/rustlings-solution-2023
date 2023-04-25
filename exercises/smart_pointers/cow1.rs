// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the TODO markers.

use std::borrow::Cow;

// Explanation: Cow's full name is Clone on write, it's actually just like the name. When it needs
//to write on something (here is write the negation of nagative numbers in input), Cow will create an
//owned copy of it and mutate.
fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v; //if input < 0 then change to mutable(Owned?)
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            // Explanation: We passed reference to construct the Cow smart pointer, and
            //there is no negative values in the "slice", so the returned slice will still
            //be references.
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected owned value1"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly.
        // In this case no mutation occurs and thus also no clone,
        // but the result is still owned because it always was.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            // Explanation: We passed actual slice ownership to the Cow smart pointer, and there
            //is no negative values in the "slice", so the returned type remains unchange.
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value2"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur.
        // In this case the call to `to_mut()` returns a reference to
        // the same data as before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            // Explanation: we passed actual slice ownership to the Cow smart pointer, and there
            //is no negative values in the "slice", so the returned type remains unchange.
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value3"),
        }
    }
}
