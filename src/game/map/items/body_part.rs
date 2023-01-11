use super::super::{
    super::{
        bodies::{Freshness, OrganData},
        races::{age_name, Gender},
    },
    item::{ItemInteract, ItemView},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BodyPart {
    #[serde(rename = "n")]
    pub name: String,
    // like "second head", TODO: probably rename to ID or "key"
    #[serde(rename = "t")]
    pub typ: BodyPartType,
    #[serde(rename = "d")]
    pub data: OrganData,
    #[serde(rename = "o")]
    pub outside: Vec<BodyPart>,
    #[serde(rename = "i")]
    pub inside: Vec<BodyPart>,
}

impl BodyPart {
    pub fn new<S: Into<String>>(name: S, typ: BodyPartType, data: OrganData) -> Self {
        Self {
            name: name.into(),
            typ,
            data,
            outside: Vec::default(),
            inside: Vec::default(),
        }
    }

    pub fn with_inside(mut self, inside: Vec<BodyPart>) -> Self {
        self.inside = inside;
        self
    }

    pub fn with_outside(mut self, outside: Vec<BodyPart>) -> Self {
        self.outside = outside;
        self
    }

    pub fn freshness(&self) -> Freshness {
        self.data.freshness
    }

    pub fn age_name(&self) -> String {
        age_name(
            self.data.race,
            self.data.age,
            Some(Gender::from(self.data.sex)),
        )
    }
}

impl ItemView for BodyPart {
    fn name(&self) -> String {
        let age_name = self.age_name();
        if matches!(
            self,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    freshness: Freshness::Skeletal,
                    ..
                },
                ..
            }
        ) {
            return format!("{age_name} skull");
        }
        let adjective = self.freshness().adjective();
        let name = match self.typ {
            BodyPartType::Head => "head",
            BodyPartType::Eye => "eye",
            BodyPartType::Nose => "nose",
            BodyPartType::Mouth => "mouth",
            BodyPartType::Ear => "ear",
            BodyPartType::Brain => "brain",
            BodyPartType::Torso => "torso",
            BodyPartType::Heart => "heart",
            BodyPartType::Stomach => "stomach",
            BodyPartType::Lung => "lung",
            BodyPartType::Kidney => "kidney",
            BodyPartType::Liver => "liver",
            BodyPartType::Intestines => "intestines",
            BodyPartType::LeftArm => "left arm",
            BodyPartType::LeftHand => "left hand",
            BodyPartType::RightArm => "right arm",
            BodyPartType::RightHand => "right hand",
            BodyPartType::LeftLeg => "left leg",
            BodyPartType::LeftFoot => "left foot",
            BodyPartType::RightLeg => "right leg",
            BodyPartType::RightFoot => "right foot",
            BodyPartType::Tail => "tail",
            BodyPartType::Jaws => "jaws",
        };
        format!("{adjective} {age_name} {name}")
    }

    fn looks_like(&self) -> &'static str {
        "flesh"
    }
}

impl ItemInteract for BodyPart {
    fn mass(&self) -> u32 {
        // TODO: use body part type, race, sex and body size
        100
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodyPartType {
    Head,
    Eye,
    Nose,
    Mouth,
    Jaws,
    Ear,
    Brain,
    Torso,
    Heart,
    Stomach,
    Lung,
    Kidney,
    Liver,
    Intestines,
    LeftArm,
    // TODO: shoulders
    LeftHand,
    // TODO: fingers
    RightArm,
    RightHand,
    LeftLeg,
    LeftFoot,
    RightLeg,
    RightFoot,
    Tail,
}
