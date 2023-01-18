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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::a;

    #[test_case("cat", "a cat")]
    #[test_case("axe", "an axe")]
    #[test_case("hour", "an hour")]
    #[test_case("honest man", "an honest man")]
    #[test_case("unit", "a unit")]
    fn test_a(word: &str, result: &str) {
        assert_eq!(a(word), result);
    }
}
