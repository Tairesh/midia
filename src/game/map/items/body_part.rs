use super::super::{
    super::{
        animals::FurColor,
        bodies::{Freshness, OrganData, Sex},
        human::{age_name, Gender, HairColor, Personality, SkinTone},
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
    #[serde(rename = "o")]
    pub outside: Vec<BodyPart>,
    #[serde(rename = "i")]
    pub inside: Vec<BodyPart>,
}

impl BodyPart {
    pub fn new<S: Into<String>>(name: S, typ: BodyPartType) -> Self {
        Self {
            name: name.into(),
            typ,
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

    pub fn organ_data(&self) -> &OrganData {
        match &self.typ {
            BodyPartType::HumanHead(data, ..)
            | BodyPartType::HumanEye(data)
            | BodyPartType::HumanNose(data, ..)
            | BodyPartType::HumanMouth(data, ..)
            | BodyPartType::HumanEar(data, ..)
            | BodyPartType::HumanBrain(data, ..)
            | BodyPartType::HumanTorso(data, ..)
            | BodyPartType::HumanHeart(data, ..)
            | BodyPartType::HumanStomach(data, ..)
            | BodyPartType::HumanLung(data, ..)
            | BodyPartType::HumanKidney(data, ..)
            | BodyPartType::HumanLiver(data, ..)
            | BodyPartType::HumanIntestines(data, ..)
            | BodyPartType::HumanLeftArm(data, ..)
            | BodyPartType::HumanLeftHand(data, ..)
            | BodyPartType::HumanRightArm(data, ..)
            | BodyPartType::HumanRightHand(data, ..)
            | BodyPartType::HumanLeftLeg(data, ..)
            | BodyPartType::HumanLeftFoot(data, ..)
            | BodyPartType::HumanRightLeg(data, ..)
            | BodyPartType::HumanRightFoot(data, ..)
            | BodyPartType::DogHead(data, ..)
            | BodyPartType::DogMaw(data, ..)
            | BodyPartType::DogEye(data, ..)
            | BodyPartType::DogNose(data, ..)
            | BodyPartType::DogEar(data, ..)
            | BodyPartType::DogBrain(data, ..)
            | BodyPartType::DogTorso(data, ..)
            | BodyPartType::DogHeart(data, ..)
            | BodyPartType::DogStomach(data, ..)
            | BodyPartType::DogLung(data, ..)
            | BodyPartType::DogKidney(data, ..)
            | BodyPartType::DogLiver(data, ..)
            | BodyPartType::DogIntestines(data, ..)
            | BodyPartType::DogPaw(data, ..)
            | BodyPartType::DogTail(data, ..) => data,
        }
    }

    pub fn freshness(&self) -> Freshness {
        self.organ_data().freshness
    }

    pub fn sex(&self) -> Option<Sex> {
        match self.typ {
            BodyPartType::HumanHead(.., sex)
            | BodyPartType::HumanMouth(.., sex)
            | BodyPartType::HumanTorso(.., sex)
            | BodyPartType::HumanLeftArm(.., sex)
            | BodyPartType::HumanLeftHand(.., sex)
            | BodyPartType::HumanRightArm(.., sex)
            | BodyPartType::HumanRightHand(.., sex)
            | BodyPartType::HumanLeftLeg(.., sex)
            | BodyPartType::HumanLeftFoot(.., sex)
            | BodyPartType::HumanRightLeg(.., sex)
            | BodyPartType::HumanRightFoot(.., sex)
            | BodyPartType::DogTorso(.., sex)
            | BodyPartType::DogHead(.., sex) => Some(sex),
            _ => None,
        }
    }

    // TODO: AnimalType enum probably
    pub fn is_human(&self) -> bool {
        matches!(
            self.typ,
            BodyPartType::HumanHead(_, _, _, _)
                | BodyPartType::HumanEye(_)
                | BodyPartType::HumanNose(_, _)
                | BodyPartType::HumanMouth(_, _, _)
                | BodyPartType::HumanEar(_, _)
                | BodyPartType::HumanBrain(_, _)
                | BodyPartType::HumanTorso(_, _, _, _)
                | BodyPartType::HumanHeart(_)
                | BodyPartType::HumanStomach(_)
                | BodyPartType::HumanLung(_)
                | BodyPartType::HumanKidney(_)
                | BodyPartType::HumanLiver(_)
                | BodyPartType::HumanIntestines(_)
                | BodyPartType::HumanLeftArm(_, _, _)
                | BodyPartType::HumanLeftHand(_, _, _)
                | BodyPartType::HumanRightArm(_, _, _)
                | BodyPartType::HumanRightHand(_, _, _)
                | BodyPartType::HumanLeftLeg(_, _, _)
                | BodyPartType::HumanLeftFoot(_, _, _)
                | BodyPartType::HumanRightLeg(_, _, _)
                | BodyPartType::HumanRightFoot(_, _, _)
        )
    }

    pub fn age_name(&self) -> &str {
        let gender = self.sex().map(Gender::from);
        if self.is_human() {
            age_name(
                self.organ_data().age,
                if let Some(gender) = &gender {
                    Some(gender)
                } else {
                    None
                },
            )
        } else if self.organ_data().age < 1 {
            "puppet"
        } else {
            "dog"
        }
    }
}

impl ItemView for BodyPart {
    fn name(&self) -> String {
        let age_name = self.age_name();
        if matches!(
            self.typ,
            BodyPartType::HumanHead(
                OrganData {
                    freshness: Freshness::Skeletal,
                    ..
                },
                ..
            )
        ) {
            return format!("{age_name} skull");
        }
        let adjective = self.freshness().adjective();
        let name = match self.typ {
            BodyPartType::HumanHead(_, _, _, _) | BodyPartType::DogHead(_, _, _) => "head",
            BodyPartType::HumanEye(_) | BodyPartType::DogEye(_) => "eye",
            BodyPartType::HumanNose(_, _) | BodyPartType::DogNose(_, _) => "nose",
            BodyPartType::HumanMouth(_, _, _) => "mouth",
            BodyPartType::HumanEar(_, _) | BodyPartType::DogEar(_, _) => "ear",
            BodyPartType::HumanBrain(_, _) | BodyPartType::DogBrain(_) => "brain",
            BodyPartType::HumanTorso(_, _, _, _) | BodyPartType::DogTorso(_, _, _) => "torso",
            BodyPartType::HumanHeart(_) | BodyPartType::DogHeart(_) => "heart",
            BodyPartType::HumanStomach(_) | BodyPartType::DogStomach(_) => "stomach",
            BodyPartType::HumanLung(_) | BodyPartType::DogLung(_) => "lung",
            BodyPartType::HumanKidney(_) | BodyPartType::DogKidney(_) => "kidney",
            BodyPartType::HumanLiver(_) | BodyPartType::DogLiver(_) => "liver",
            BodyPartType::HumanIntestines(_) | BodyPartType::DogIntestines(_) => "intestines",
            BodyPartType::HumanLeftArm(_, _, _) => "left arm",
            BodyPartType::HumanLeftHand(_, _, _) => "left hand",
            BodyPartType::HumanRightArm(_, _, _) => "right arm",
            BodyPartType::HumanRightHand(_, _, _) => "right hand",
            BodyPartType::HumanLeftLeg(_, _, _) => "left leg",
            BodyPartType::HumanLeftFoot(_, _, _) => "left foot",
            BodyPartType::HumanRightLeg(_, _, _) => "right leg",
            BodyPartType::HumanRightFoot(_, _, _) => "right foot",
            BodyPartType::DogMaw(_, _) => "maw",
            BodyPartType::DogPaw(_, _) => "paw",
            BodyPartType::DogTail(_, _) => "tail",
        };
        format!("{adjective} {age_name} {name}")
    }

    fn looks_like(&self) -> &'static str {
        "flesh"
    }
}

impl ItemInteract for BodyPart {
    fn mass(&self) -> u32 {
        // TODO: use Sex and BodySize
        match self.typ {
            BodyPartType::HumanHead(..) => 3_500,
            BodyPartType::HumanEye(..) | BodyPartType::DogEye(..) => 8,
            BodyPartType::HumanNose(..) => 60,
            BodyPartType::HumanMouth(..) => 200,
            BodyPartType::HumanEar(..) => 50,
            BodyPartType::HumanBrain(..) => 1_400,
            BodyPartType::HumanTorso(..) => 25_000,
            BodyPartType::HumanHeart(..) => 250,
            BodyPartType::HumanStomach(..) => 125,
            BodyPartType::HumanLung(..) => 600,
            BodyPartType::HumanKidney(..) => 100,
            BodyPartType::HumanLiver(..) => 1_500,
            BodyPartType::HumanIntestines(..) => 2_000,
            BodyPartType::HumanLeftArm(..) | BodyPartType::HumanRightArm(..) => 3_000,
            BodyPartType::HumanLeftHand(..) | BodyPartType::HumanRightHand(..) => 500,
            BodyPartType::HumanLeftLeg(..) | BodyPartType::HumanRightLeg(..) => 10_000,
            BodyPartType::HumanLeftFoot(..) | BodyPartType::HumanRightFoot(..) => 750,
            BodyPartType::DogHead(..) => 4_000,
            BodyPartType::DogMaw(..) => 700,
            BodyPartType::DogNose(..) => 70,
            BodyPartType::DogEar(..) => 45,
            BodyPartType::DogBrain(..) => 900,
            BodyPartType::DogTorso(..) => 30_000,
            BodyPartType::DogHeart(..) => 260,
            BodyPartType::DogStomach(..) => 130,
            BodyPartType::DogLung(..) => 650,
            BodyPartType::DogKidney(..) => 120,
            BodyPartType::DogLiver(..) => 1_600,
            BodyPartType::DogIntestines(..) => 2_750,
            BodyPartType::DogPaw(..) => 11_000,
            BodyPartType::DogTail(..) => 2_500,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodyPartType {
    HumanHead(OrganData, HairColor, SkinTone, Sex),
    HumanEye(OrganData),
    // TODO: eye color
    HumanNose(OrganData, SkinTone),
    HumanMouth(OrganData, SkinTone, Sex),
    // TODO: jaws, teeth, beard
    HumanEar(OrganData, SkinTone),
    HumanBrain(OrganData, Personality),
    HumanTorso(OrganData, HairColor, SkinTone, Sex),
    HumanHeart(OrganData),
    HumanStomach(OrganData),
    HumanLung(OrganData),
    HumanKidney(OrganData),
    HumanLiver(OrganData),
    HumanIntestines(OrganData),
    HumanLeftArm(OrganData, SkinTone, Sex),
    // TODO: shoulders
    HumanLeftHand(OrganData, SkinTone, Sex),
    // TODO: fingers
    HumanRightArm(OrganData, SkinTone, Sex),
    HumanRightHand(OrganData, SkinTone, Sex),
    HumanLeftLeg(OrganData, SkinTone, Sex),
    HumanLeftFoot(OrganData, SkinTone, Sex),
    HumanRightLeg(OrganData, SkinTone, Sex),
    HumanRightFoot(OrganData, SkinTone, Sex),
    DogHead(OrganData, FurColor, Sex),
    DogMaw(OrganData, FurColor),
    DogEye(OrganData),
    DogNose(OrganData, FurColor),
    DogEar(OrganData, FurColor),
    DogBrain(OrganData),
    // TODO: dog-identity (Character-like struct with name)
    DogTorso(OrganData, FurColor, Sex),
    DogHeart(OrganData),
    DogStomach(OrganData),
    DogLung(OrganData),
    DogKidney(OrganData),
    DogLiver(OrganData),
    DogIntestines(OrganData),
    DogPaw(OrganData, FurColor),
    DogTail(OrganData, FurColor),
}
