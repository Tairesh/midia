#[allow(dead_code)]
const CONSONANTS: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'y', 'z',
];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const WORDS_WITH_SILENT_H: [&str; 3] = ["hour", "honest", "honor"];
const WORDS_WITH_U_OR_O: [&str; 7] = ["union", "one", "unit", "unique", "unite", "use", "usual"];

pub fn a(word: impl Into<String>) -> String {
    let word = word.into();
    if (VOWELS.contains(&word.chars().next().unwrap())
        && !WORDS_WITH_U_OR_O.iter().any(|w| word.starts_with(w)))
        || WORDS_WITH_SILENT_H.iter().any(|w| word.starts_with(w))
    {
        format!("an {word}")
    } else {
        format!("a {word}")
    }
}

pub trait Capitalize: AsRef<str> {
    /// Change first character to upper case and the rest to lower case.
    fn capitalize(&self) -> String;
}

impl<T: AsRef<str>> Capitalize for T {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("cat", "a cat")]
    #[test_case("axe", "an axe")]
    #[test_case("hour", "an hour")]
    #[test_case("honest man", "an honest man")]
    #[test_case("unit", "a unit")]
    fn test_a(word: &str, result: &str) {
        assert_eq!(a(word), result);
    }

    #[test_case("", "")]
    #[test_case("you pet the cat", "You pet the cat")]
    #[test_case("this book is called 'The Cat'", "This book is called 'The Cat'")]
    fn test_capitalize(sentence: &str, result: &str) {
        assert_eq!(sentence.capitalize(), result);
    }
}
