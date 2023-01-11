use std::collections::HashMap;

use geometry::Point;

use super::super::{
    bodies::{Body, Freshness, OrganData},
    map::items::{BodyPart, BodyPartType},
};

pub fn brain(organ_data: OrganData) -> BodyPart {
    BodyPart::new("brain", BodyPartType::Brain, organ_data)
}

pub fn eye(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left eye" } else { "right eye" },
        BodyPartType::Eye,
        organ_data,
    )
}

pub fn nose(organ_data: OrganData) -> BodyPart {
    BodyPart::new("nose", BodyPartType::Nose, organ_data)
}

pub fn mouth(organ_data: OrganData) -> BodyPart {
    BodyPart::new("mouth", BodyPartType::Mouth, organ_data)
}

#[allow(dead_code)]
pub fn jaws(organ_data: OrganData) -> BodyPart {
    BodyPart::new("jaws", BodyPartType::Jaws, organ_data)
}

pub fn ear(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left ear" } else { "right ear" },
        BodyPartType::Ear,
        organ_data,
    )
}

pub fn head(organ_data: OrganData) -> BodyPart {
    BodyPart::new("head", BodyPartType::Head, organ_data)
        .with_inside(match organ_data.freshness {
            Freshness::Fresh | Freshness::Rotten => {
                vec![brain(organ_data)]
            }
            Freshness::Skeletal => vec![],
        })
        .with_outside(match organ_data.freshness {
            Freshness::Fresh => vec![
                eye(organ_data, true),
                eye(organ_data, false),
                nose(organ_data),
                ear(organ_data, true),
                ear(organ_data, false),
                mouth(organ_data),
            ],
            Freshness::Rotten => vec![
                nose(organ_data),
                mouth(organ_data),
                ear(organ_data, true),
                ear(organ_data, false),
            ],
            Freshness::Skeletal => vec![mouth(organ_data)],
        })
}

pub fn heart(organ_data: OrganData) -> BodyPart {
    BodyPart::new("heart", BodyPartType::Heart, organ_data)
}

pub fn stomach(organ_data: OrganData) -> BodyPart {
    BodyPart::new("stomach", BodyPartType::Stomach, organ_data)
}

pub fn liver(organ_data: OrganData) -> BodyPart {
    BodyPart::new("liver", BodyPartType::Liver, organ_data)
}

pub fn intestines(organ_data: OrganData) -> BodyPart {
    BodyPart::new("intestines", BodyPartType::Intestines, organ_data)
}

pub fn lung(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left lung" } else { "right lung" },
        BodyPartType::Lung,
        organ_data,
    )
}

pub fn kidney(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left kidney" } else { "right kidney" },
        BodyPartType::Kidney,
        organ_data,
    )
}

pub fn arm(organ_data: OrganData, left: bool) -> BodyPart {
    let mut arm = BodyPart::new(
        if left { "left arm" } else { "right arm" },
        if left {
            BodyPartType::LeftArm
        } else {
            BodyPartType::RightArm
        },
        organ_data,
    );
    arm.outside.push(hand(organ_data, left));

    arm
}

pub fn hand(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left hand" } else { "right hand" },
        if left {
            BodyPartType::LeftHand
        } else {
            BodyPartType::RightHand
        },
        organ_data,
    )
}

pub fn leg(organ_data: OrganData, left: bool) -> BodyPart {
    let mut leg = BodyPart::new(
        if left { "left leg" } else { "right leg" },
        if left {
            BodyPartType::LeftLeg
        } else {
            BodyPartType::RightLeg
        },
        organ_data,
    );
    leg.outside.push(foot(organ_data, left));

    leg
}

pub fn foot(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left foot" } else { "right foot" },
        if left {
            BodyPartType::LeftFoot
        } else {
            BodyPartType::RightFoot
        },
        organ_data,
    )
}

pub fn torso(organ_data: OrganData) -> BodyPart {
    BodyPart::new("torso", BodyPartType::Torso, organ_data)
        .with_inside(match organ_data.freshness {
            Freshness::Fresh | Freshness::Rotten => vec![
                heart(organ_data),
                lung(organ_data, true),
                lung(organ_data, false),
                stomach(organ_data),
                kidney(organ_data, true),
                kidney(organ_data, false),
                liver(organ_data),
                intestines(organ_data),
            ],
            Freshness::Skeletal => vec![],
        })
        .with_outside(vec![
            head(organ_data),
            arm(organ_data, true),
            arm(organ_data, false),
            leg(organ_data, true),
            leg(organ_data, false),
        ])
}

pub fn body(organ_data: OrganData) -> Body {
    Body::new(HashMap::from([(Point::new(0, 0), torso(organ_data))]))
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::bodies::Body;
    use crate::game::races::{FurColor, Personality};

    use super::{
        super::{
            super::{
                bodies::{BodySize, Freshness, OrganData, Sex},
                map::items::{BodyPart, BodyPartType},
                ItemView,
            },
            tests::personality::{dead_boy, old_queer, tester_girl},
            Race, SkinTone,
        },
        body, head, torso,
    };

    pub fn human_centipede(personalities: Vec<Personality>) -> Body {
        let parts = personalities
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                (
                    Point::new(0, i as i32),
                    torso(OrganData::new(&p, Freshness::Fresh)),
                )
            })
            .collect();
        Body::new(parts)
    }

    #[test]
    fn test_fresh_head() {
        let character = tester_girl();
        let head = head(OrganData::new(&character, Freshness::Fresh));
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    age: 15,
                    alive: true,
                    size: BodySize::Small,
                    freshness: Freshness::Fresh,
                    sex: Sex::Female,
                    skin_tone: SkinTone::WarmIvory,
                    fur_color: Some(FurColor::Ginger),
                    race: Race::Gazan,
                },
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain,
                data: OrganData {
                    freshness: Freshness::Fresh,
                    age: 15,
                    size: BodySize::Small,
                    alive: true,
                    sex: Sex::Female,
                    skin_tone: SkinTone::WarmIvory,
                    fur_color: Some(FurColor::Ginger),
                    race: Race::Gazan,
                },
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
                        typ: BodyPartType::Eye,
                        data: OrganData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            size: BodySize::Small,
                            alive: true,
                            ..
                        },
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
                        typ: BodyPartType::Ear,
                        data: OrganData {
                            freshness: Freshness::Fresh,
                            age: 15,
                            size: BodySize::Small,
                            alive: true,
                            skin_tone: SkinTone::WarmIvory,
                            ..
                        },
                        ..
                    }
                ))
                .count()
        );
    }

    #[test]
    fn test_rotten_head() {
        let character = dead_boy();
        let head = head(OrganData::new(&character, Freshness::Rotten));
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head,
                ..
            }
        ));
        assert!(matches!(
            head.inside.iter().next(),
            Some(BodyPart {
                typ: BodyPartType::Brain,
                data: OrganData {
                    freshness: Freshness::Rotten,
                    age: 9,
                    ..
                },
                ..
            })
        ));
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear))
                .count()
        );
    }

    #[test]
    fn test_skeletal_head() {
        let character = dead_boy();
        let head = head(OrganData::new(&character, Freshness::Skeletal));
        assert_eq!("head", head.name);
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    age: 9,
                    alive: false,
                    size: BodySize::Tiny,
                    freshness: Freshness::Skeletal,
                    fur_color: Some(FurColor::Ginger),
                    skin_tone: SkinTone::Almond,
                    sex: Sex::Male,
                    race: Race::Gazan,
                },
                ..
            }
        ));
        assert!(head.inside.is_empty());
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Eye))
                .count()
        );
        assert_eq!(
            0,
            head.outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Ear))
                .count()
        );
    }

    #[test]
    fn test_fresh_torso() {
        let character = tester_girl();
        let torso = torso(OrganData::new(&character, Freshness::Fresh));
        assert_eq!("torso", torso.name);
        assert_eq!("fresh gazan girl torso", torso.name());
        assert!(matches!(
            torso,
            BodyPart {
                typ: BodyPartType::Torso,
                data: OrganData {
                    age: 15,
                    alive: true,
                    freshness: Freshness::Fresh,
                    size: BodySize::Small,
                    fur_color: Some(FurColor::Ginger),
                    skin_tone: SkinTone::WarmIvory,
                    sex: Sex::Female,
                    race: Race::Gazan,
                },
                ..
            }
        ));
        assert_eq!(
            1,
            torso
                .inside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Heart))
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
                .filter(|bp| matches!(bp.typ, BodyPartType::Lung))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::Head))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftArm))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightArm))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::LeftLeg))
                .count()
        );
        assert_eq!(
            1,
            torso
                .outside
                .iter()
                .filter(|bp| matches!(bp.typ, BodyPartType::RightLeg))
                .count()
        );
    }

    #[test]
    fn test_old_man_body() {
        let character = old_queer();
        let body = body(OrganData::new(&character, Freshness::Fresh));
        let torso = body.parts.get(&Point::new(0, 0)).unwrap();
        let head = torso.outside.first().unwrap();
        assert!(matches!(
            head,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    age: 99,
                    alive: true,
                    freshness: Freshness::Fresh,
                    size: BodySize::Large,
                    fur_color: None,
                    skin_tone: SkinTone::Almond,
                    race: Race::Bug,
                    sex: Sex::Undefined,
                },
                ..
            }
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
                typ: BodyPartType::Head,
                data: OrganData {
                    race: Race::Gazan,
                    age: 9,
                    alive: false,
                    size: BodySize::Tiny,
                    freshness: Freshness::Fresh,
                    sex: Sex::Male,
                    skin_tone: SkinTone::Almond,
                    fur_color: Some(FurColor::Ginger),
                },
                ..
            }
        ));
        let torso2 = body.parts.get(&Point::new(0, 1)).unwrap();
        let head2 = torso2.outside.first().unwrap();
        assert_eq!("head", head2.name);
        assert!(matches!(
            head2,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    age: 9,
                    alive: false,
                    size: BodySize::Tiny,
                    freshness: Freshness::Fresh,
                    fur_color: Some(FurColor::Ginger),
                    skin_tone: SkinTone::Almond,
                    sex: Sex::Male,
                    race: Race::Gazan,
                },
                ..
            }
        ));
        let torso3 = body.parts.get(&Point::new(0, 2)).unwrap();
        let head3 = torso3.outside.first().unwrap();
        assert_eq!("head", head3.name);
        assert!(matches!(
            head3,
            BodyPart {
                typ: BodyPartType::Head,
                data: OrganData {
                    age: 9,
                    alive: false,
                    size: BodySize::Tiny,
                    freshness: Freshness::Fresh,
                    fur_color: Some(FurColor::Ginger),
                    skin_tone: SkinTone::Almond,
                    sex: Sex::Male,
                    race: Race::Gazan,
                },
                ..
            }
        ));
    }
}
