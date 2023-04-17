// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
        // Explanation: the `ParseIntError` from `total_cost` will be "return"
        //to `cost` in main().(see https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
        //and `cost` will "return" the main() with a `ParseIntError`, which is not defined in the
        //signature of main(). Hence, we need to add return type for main().
        Ok(())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        Ok(())
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
