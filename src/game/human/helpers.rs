use std::collections::HashMap;
use std::convert::TryInto;

use geometry::Point;

use super::{
    super::{
        bodies::{Body, Freshness, OrganData, Sex},
        map::items::{BodyPart, BodyPartType},
    },
    HairColor, Personality, SkinTone,
};

pub fn human_brain(organ_data: OrganData, character: Personality) -> BodyPart {
    BodyPart::new("brain", BodyPartType::HumanBrain(organ_data, character))
}

pub fn human_eye(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left eye" } else { "right eye" },
        BodyPartType::HumanEye(organ_data),
    )
}

pub fn human_nose(organ_data: OrganData, skin_tone: SkinTone) -> BodyPart {
    BodyPart::new("nose", BodyPartType::HumanNose(organ_data, skin_tone))
}

pub fn human_mouth(organ_data: OrganData, skin_tone: SkinTone, sex: Sex) -> BodyPart {
    BodyPart::new(
        "mouth",
        BodyPartType::HumanMouth(organ_data, skin_tone, sex),
    )
}

pub fn human_ear(organ_data: OrganData, skin_tone: SkinTone, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left ear" } else { "right ear" },
        BodyPartType::HumanEar(organ_data, skin_tone),
    )
}

pub fn human_head(personality: &Personality, freshness: Freshness) -> BodyPart {
    let organ_data = OrganData::new(personality, freshness);
    let sex = (&personality.mind.gender).try_into().unwrap_or_default();
    let hair_color = if personality.appearance.age < 50 {
        personality.appearance.hair_color
    } else {
        HairColor::Gray
    };
    let skin_tone = personality.appearance.skin_tone;
    BodyPart::new(
        "head",
        BodyPartType::HumanHead(organ_data.clone(), hair_color, skin_tone, sex),
    )
    .with_inside(match freshness {
        Freshness::Fresh | Freshness::Rotten => {
            vec![human_brain(organ_data.clone(), personality.clone())]
        }
        Freshness::Skeletal => vec![],
    })
    .with_outside(match freshness {
        Freshness::Fresh => vec![
            human_eye(organ_data.clone(), true),
            human_eye(organ_data.clone(), false),
            human_nose(organ_data.clone(), skin_tone),
            human_ear(organ_data.clone(), skin_tone, true),
            human_ear(organ_data.clone(), skin_tone, false),
            human_mouth(organ_data, skin_tone, sex),
        ],
        Freshness::Rotten => vec![
            human_nose(organ_data.clone(), skin_tone),
            human_mouth(organ_data.clone(), skin_tone, sex),
            human_ear(organ_data.clone(), skin_tone, true),
            human_ear(organ_data, skin_tone, false),
        ],
        Freshness::Skeletal => vec![human_mouth(organ_data, skin_tone, sex)],
    })
}

pub fn human_heart(organ_data: OrganData) -> BodyPart {
    BodyPart::new("heart", BodyPartType::HumanHeart(organ_data))
}

pub fn human_stomach(organ_data: OrganData) -> BodyPart {
    BodyPart::new("stomach", BodyPartType::HumanStomach(organ_data))
}

pub fn human_liver(organ_data: OrganData) -> BodyPart {
    BodyPart::new("liver", BodyPartType::HumanLiver(organ_data))
}

pub fn human_intestines(organ_data: OrganData) -> BodyPart {
    BodyPart::new("intestines", BodyPartType::HumanIntestines(organ_data))
}

pub fn human_lung(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left lung" } else { "right lung" },
        BodyPartType::HumanLung(organ_data),
    )
}

pub fn human_kidney(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left kidney" } else { "right kidney" },
        BodyPartType::HumanKidney(organ_data),
    )
}

pub fn human_arm(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    let mut arm = BodyPart::new(
        if left { "left arm" } else { "right arm" },
        if left {
            BodyPartType::HumanLeftArm(organ_data.clone(), skin_tone, sex)
        } else {
            BodyPartType::HumanRightArm(organ_data.clone(), skin_tone, sex)
        },
    );
    arm.outside
        .push(human_hand(organ_data, skin_tone, sex, left));

    arm
}

pub fn human_hand(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left hand" } else { "right hand" },
        if left {
            BodyPartType::HumanLeftHand(organ_data, skin_tone, sex)
        } else {
            BodyPartType::HumanRightHand(organ_data, skin_tone, sex)
        },
    )
}

pub fn human_leg(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    let mut leg = BodyPart::new(
        if left { "left leg" } else { "right leg" },
        if left {
            BodyPartType::HumanLeftLeg(organ_data.clone(), skin_tone, sex)
        } else {
            BodyPartType::HumanRightLeg(organ_data.clone(), skin_tone, sex)
        },
    );
    leg.outside
        .push(human_foot(organ_data, skin_tone, sex, left));

    leg
}

pub fn human_foot(organ_data: OrganData, skin_tone: SkinTone, sex: Sex, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left foot" } else { "right foot" },
        if left {
            BodyPartType::HumanLeftFoot(organ_data, skin_tone, sex)
        } else {
            BodyPartType::HumanRightFoot(organ_data, skin_tone, sex)
        },
    )
}

pub fn human_torso(personality: &Personality, freshness: Freshness) -> BodyPart {
    let organ_data = OrganData::new(personality, freshness);
    let skin_tone = personality.appearance.skin_tone;
    let hair_color = personality.appearance.hair_color;
    let sex = (&personality.mind.gender).try_into().unwrap_or_default();
    BodyPart::new(
        "torso",
        BodyPartType::HumanTorso(organ_data.clone(), hair_color, skin_tone, sex),
    )
    .with_inside(match freshness {
        Freshness::Fresh | Freshness::Rotten => vec![
            human_heart(organ_data.clone()),
            human_lung(organ_data.clone(), true),
            human_lung(organ_data.clone(), false),
            human_stomach(organ_data.clone()),
            human_kidney(organ_data.clone(), true),
            human_kidney(organ_data.clone(), false),
            human_liver(organ_data.clone()),
            human_intestines(organ_data.clone()),
        ],
        Freshness::Skeletal => vec![],
    })
    .with_outside(vec![
        human_head(personality, freshness),
        human_arm(organ_data.clone(), skin_tone, sex, true),
        human_arm(organ_data.clone(), skin_tone, sex, false),
        human_leg(organ_data.clone(), skin_tone, sex, true),
        human_leg(organ_data, skin_tone, sex, false),
    ])
}

pub fn human_body(personality: &Personality, freshness: Freshness) -> Body {
    let parts = HashMap::from([(Point::new(0, 0), human_torso(personality, freshness))]);
    Body::new(parts)
}

#[allow(dead_code)]
pub fn human_centipede(personalities: Vec<Personality>) -> Body {
    let parts = personalities
        .into_iter()
        .enumerate()
        .map(|(i, p)| (Point::new(0, i as i32), human_torso(&p, Freshness::Fresh)))
        .collect();
    Body::new(parts)
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use super::{
        super::{
            super::{
                bodies::{BodySize, Freshness, OrganData, Sex},
                map::items::{BodyPart, BodyPartType},
                ItemView,
            },
            tests::personality::{dead_boy, old_queer, tester_girl},
            Appearance, Gender, HairColor, MainHand, Mind, Personality, SkinTone,
        },
        human_body, human_centipede, human_head, human_torso,
    };

    #[test]
    fn test_fresh_head() {
        let character = tester_girl();
        let head = human_head(&character, Freshness::Fresh);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 15,
                        alive: true,
                        size: BodySize::Small,
                        freshness: Freshness::Fresh,
                    },
                    HairColor::Ginger,
                    SkinTone::WarmIvory,
                    Sex::Female,
                ),
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::HumanBrain(
                    OrganData {
                        freshness: Freshness::Fresh,
                        age: 15,
                        size: BodySize::Small,
                        alive: true,
                    },
                    Personality {
                        appearance: Appearance {
                            age: 15,
                            hair_color: HairColor::Ginger,
                            body_size: BodySize::Small,
                            skin_tone: SkinTone::WarmIvory,
                        },
                        mind: Mind {
                            alive: true,
                            gender: Gender::Female,
                            main_hand: MainHand::Left,
                            ..
                        }
                    }
                ),
                ..
            })
        ));
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(
                    bp,
                    BodyPart {
                        typ: BodyPartType::HumanEye(OrganData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            size: BodySize::Small,
                            alive: true,
                        }),
                        ..
                    }
                ))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(
                    bp,
                    BodyPart {
                        typ: BodyPartType::HumanEar(
                            OrganData {
                                freshness: Freshness::Fresh,
                                age: 15,
                                size: BodySize::Small,
                                alive: true,
                            },
                            SkinTone::WarmIvory
                        ),
                        ..
                    }
                ))
                .count()
        );
    }

    #[test]
    fn test_rotten_head() {
        let character = dead_boy();
        let head = human_head(&character, Freshness::Rotten);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Rotten,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::HumanBrain(
                    OrganData {
                        freshness: Freshness::Rotten,
                        age: 9,
                        ..
                    },
                    Personality {
                        mind: Mind {
                            gender: Gender::Male,
                            ..
                        },
                        ..
                    }
                ),
                ..
            })
        ));
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanEye(..)))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanEar(..)))
                .count()
        );
    }

    #[test]
    fn test_skeletal_head() {
        let character = dead_boy();
        let head = human_head(&character, Freshness::Skeletal);
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Skeletal,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        assert!(head.inside.is_empty());
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanEye(..)))
                .count()
        );
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanEar(..)))
                .count()
        );
    }

    #[test]
    fn test_fresh_torso() {
        let character = tester_girl();
        let torso = human_torso(&character, Freshness::Fresh);
        assert_eq!("torso", torso.name);
        assert_eq!("fresh girl torso", torso.name());
        assert!(matches!(
            torso,
            BodyPart {
                typ: BodyPartType::HumanTorso(
                    OrganData {
                        age: 15,
                        alive: true,
                        freshness: Freshness::Fresh,
                        size: BodySize::Small,
                    },
                    HairColor::Ginger,
                    SkinTone::WarmIvory,
                    Sex::Female,
                ),
                ..
            }
        ));
        assert_eq!(
            1,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanHeart(..)))
                .count()
        );
        assert_eq!(
            // А в каждом человеке есть два танцора: правое и левое.
            // Один танцор - правое, другой - левое.
            // Два легких танцора. Два легких. Правое легкое и левое.
            // В каждом человеке два танцора - его правое и левое легкое.
            // Легкие танцуют, и человек получает кислород.
            2,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanLung(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanHead(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanLeftArm(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanRightArm(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanLeftLeg(..)))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::HumanRightLeg(..)))
                .count()
        );
    }

    #[test]
    fn test_old_man_body() {
        let character = old_queer();
        let body = human_body(&character, Freshness::Fresh);
        let torso = body.parts.get(&Point::new(0, 0)).unwrap();
        let head = torso.outside.first().unwrap();
        assert!(matches!(
            head.typ,
            BodyPartType::HumanHead(
                OrganData {
                    freshness: Freshness::Fresh,
                    age: 75,
                    size: BodySize::Large,
                    alive: true,
                },
                HairColor::Gray,
                SkinTone::Almond,
                Sex::Female
            )
        ));
    }

    #[test]
    fn test_human_centipede() {
        let body = human_centipede(vec![dead_boy(), dead_boy(), dead_boy()]);

        let torso1 = body.parts.get(&Point::new(0, 0)).unwrap();
        let head1 = torso1.outside.first().unwrap();
        assert_eq!("head", head1.name);
        assert!(matches!(
            head1,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Fresh,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        let torso2 = body.parts.get(&Point::new(0, 1)).unwrap();
        let head2 = torso2.outside.first().unwrap();
        assert_eq!("head", head2.name);
        assert!(matches!(
            head2,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Fresh,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
        let torso3 = body.parts.get(&Point::new(0, 2)).unwrap();
        let head3 = torso3.outside.first().unwrap();
        assert_eq!("head", head3.name);
        assert!(matches!(
            head3,
            BodyPart {
                typ: BodyPartType::HumanHead(
                    OrganData {
                        age: 9,
                        alive: false,
                        size: BodySize::Tiny,
                        freshness: Freshness::Fresh,
                    },
                    HairColor::Black,
                    SkinTone::Almond,
                    Sex::Male,
                ),
                ..
            }
        ));
    }
}
