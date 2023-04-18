// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // Explanation: found this code online, `chain()` chains two iterators
        //together, `collect()` turns iterator into the original type.
        // Some(first) => first.to_upper_case().chain().collect(),

        // A more efficient answer: More efficient as we operates more on string
        //slices instead of iterator. Now rust seems to have a lot of magic to me...
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let mut capitalized_words = *words; //
    // capitalized_words.map(|word| capitalize_first(word));
    // capitalized_words

    // Explantion: this is a bit like magic to me acutally. `iter()` turns
    //reference to a iterator? Can any sequence of data turn into iter??
    //Then apply map and then collct the iterator.
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // Explanation: this is the method of join Vec in rust
    capitalize_words_vector(words).join("")

    // TODO: Question: Why Vector is not able to turn to iterator？
    // capitalize_words_vector
    //     .iter() //nor into_iter() works
    //     .map(|word| word.to_owned)
    //     .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
