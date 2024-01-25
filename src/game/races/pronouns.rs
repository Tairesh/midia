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
        self == Pronouns::Plural || self == Pronouns::TheyThem
    }

    pub fn pseudo_plural(self) -> bool {
        self == Pronouns::TheyThem
    }

    pub fn third_person(self) -> bool {
        self != Pronouns::YouYour
    }

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
