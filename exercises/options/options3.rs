// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // Some(&p) => println!("Co-ordinates are {},{} ", p.x, p.y), //ref p is a reference points to variable p(p is pattern matched by y), p.x auto dereferenced the reference p
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        // ref p is a reference points to variable p(p is pattern matched by y), p.x auto dereferenced the reference p(Deref trait).
        // ref p => &y
        // &p => cannot pass pattern matching.
        //Because y is a Option<Point>, which the value of y can only be Option<Point> or None, it is impossible to be Option<&Point>(point refereces), ref p is to match y first, then add a reference.
        // In order to understand the differences between ref and &, you probably wants to
        //look at this stack overflow answer: https://stackoverflow.com/questions/34717001/whats-the-difference-between-ref-and-when-assigning-a-variable-from-a-referen
        //or ask chatgpt
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
