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
        format!("an {}", word)
    } else {
        format!("a {}", word)
    }
}

#[cfg(test)]
mod tests {
    use super::a;

    #[test]
    fn test_a() {
        assert_eq!("a cat", a("cat"));
        assert_eq!("an axe", a("axe"));
        assert_eq!("an hour", a("hour"));
        assert_eq!("an honest man", a("honest man"));
        assert_eq!("a unit", a("unit"));
    }
}
