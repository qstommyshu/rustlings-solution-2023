// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); //"" creates string slice, String::from("") creates string, which is modifiable
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); //to_owned() also creates a String as string slice cannot be owned
    string_slice("nice weather".into()); //into is reciprical of ::from, which reverse String::from
    string(format!("Interpolation {}", "Station")); //format returns a String, as String is growable??
    string_slice(&String::from("abc")[0..1]); // this is string slice, [] denotes slice of the String
    string_slice("  hello there ".trim()); //sure, hard to remember
    string("Happy Monday!".to_string().replace("Mon", "Tues")); //source is String, so replace return String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //String.to_lowercase -> String
}
