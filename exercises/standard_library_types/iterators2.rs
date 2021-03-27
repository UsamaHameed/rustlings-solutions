// iterators2.rs
// In this module, you'll learn some of the unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => String::from(first.to_uppercase().to_string() + &input[1..]),
    }
}

// pub trait Input<'a> { 
//     fn to_vec(self) -> Vec<&'a str>; 
// }

// impl<'a> Input<'a> for &'a str {
//    fn to_vec(self) -> Vec<&'a str> { vec![self] }
// }

// impl<'a> Input<'a> for Vec<&'a str> {
//    fn to_vec(self) -> Vec<&'a str> { self }
// }

// pub fn capitalize_first<'a>(input: impl Input<'a>) -> Vec<String> {
//     let words = input.to_vec().iter().map(|word| { 
//         let mut c = word.chars();
//         match c.next() {
//             None => String::new(),
//             Some(first) => String::from(first.to_uppercase().to_string() + &word[1..]),
//         }
//     }).collect::<Vec<String>>();

//     words
// }

// struct OneOrMore<'a>(Vec<&'a str>);

// impl<'a> From<&'a str> for OneOrMore<'a> {
//     fn from(v: &'a str) -> Self {
//         Self(vec![v])
//     }
// }

// impl<'a> From<Vec<&'a str>> for OneOrMore<'a> {
//     fn from(v: Vec<&'a str>) -> Self {
//         Self(v)
//     }
// }

// pub fn capitalize_first<'a, T>(input: T) -> T where T: Into<OneOrMore<'a>> {
//     let words = input.iter().map(|word| { 
//         let mut c = word.chars();
//         match c.next() {
//             None => String::new(),
//             Some(first) => String::from(first.to_uppercase().to_string() + &word[1..]),
//         }
//     }).collect::<Vec<String>>();

//     words
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.iter().map(|word| { 
            capitalize_first(word)
        }).collect::<Vec<String>>();
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = words.iter().map(|word| { 
            capitalize_first(word)
        }).collect::<Vec<String>>().join("");
        assert_eq!(capitalized_words, "Hello World");
    }
}
