#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Pronouns {
    YouYour,
    ItIts,
    TheyThem,
    Plural,
    HeHim,
    SheHer,
    XeXem,
}

impl Pronouns {
    pub fn verb_ends_with_s(self) -> bool {
        self.third_person() && !self.plural()
    }

    pub fn plural(self) -> bool {
        self == Pronouns::Plural
    }

    pub fn third_person(self) -> bool {
        self != Pronouns::YouYour
    }

    pub fn is_are(self) -> &'static str {
        if self.plural() || self == Pronouns::YouYour {
            "are"
        } else {
            "is"
        }
    }

    /// Returns the pronoun's subjective form, like in "*She* went to Grit Gate."
    pub fn subjective(self) -> &'static str {
        match self {
            Pronouns::YouYour => "you",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem | Pronouns::Plural => "they",
            Pronouns::HeHim => "he",
            Pronouns::SheHer => "she",
            Pronouns::XeXem => "xe",
        }
    }

    /// Returns the pronoun's objective form, like in "You are water-bonded with *him*."
    pub fn objective(self) -> &'static str {
        match self {
            Pronouns::YouYour => "you",
            Pronouns::ItIts => "it",
            Pronouns::TheyThem | Pronouns::Plural => "them",
            Pronouns::HeHim => "him",
            Pronouns::SheHer => "her",
            Pronouns::XeXem => "xem",
        }
    }

    /// Returns the pronoun's possessive form, like in "*Her* Issachar rifle rusted."
    pub fn possessive_adjective(self) -> &'static str {
        match self {
            Pronouns::YouYour => "your",
            Pronouns::ItIts => "its",
            Pronouns::TheyThem | Pronouns::Plural => "their",
            Pronouns::HeHim => "his",
            Pronouns::SheHer => "her",
            Pronouns::XeXem => "xyr",
        }
    }

    /// Returns the pronoun's possessive form, like in "The rifle is *hers*."
    pub fn substantive_possessive(self) -> &'static str {
        match self {
            Pronouns::YouYour => "yours",
            Pronouns::ItIts => "its",
            Pronouns::TheyThem | Pronouns::Plural => "theirs",
            Pronouns::HeHim => "his",
            Pronouns::SheHer => "hers",
            Pronouns::XeXem => "xyrs",
        }
    }

    /// Returns the pronoun's reflexive form, like in "He can only blame *himself* for eating the Cloaca Surprise."
    pub fn reflexive(self) -> &'static str {
        match self {
            Pronouns::YouYour => "yourself",
            Pronouns::ItIts => "itself",
            Pronouns::TheyThem => "themself",
            Pronouns::Plural => "themselves",
            Pronouns::HeHim => "himself",
            Pronouns::SheHer => "herself",
            Pronouns::XeXem => "xemself",
        }
    }
}
