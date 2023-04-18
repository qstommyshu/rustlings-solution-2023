// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

// Explanation: this question is just the syntax of lifetime for structs that holds
//references. I feel like the question just generally become much easier after errors6
//for some reason, most of them are just straight forward from book, doesn't quite
//require me to actually understand the concept. Or maybe I became better?
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
