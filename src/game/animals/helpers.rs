#![allow(dead_code)]

use std::collections::HashMap;

use geometry::Point;

use super::{
    super::{
        bodies::{Body, BodySize, Freshness, OrganData, Sex},
        map::items::{BodyPart, BodyPartType},
    },
    FurColor,
};

pub fn dog_brain(organ_data: OrganData) -> BodyPart {
    BodyPart::new("brain", BodyPartType::DogBrain(organ_data))
}

pub fn dog_nose(organ_data: OrganData, fur_color: FurColor) -> BodyPart {
    BodyPart::new("nose", BodyPartType::DogNose(organ_data, fur_color))
}

pub fn dog_maw(organ_data: OrganData, fur_color: FurColor) -> BodyPart {
    BodyPart::new("maw", BodyPartType::DogMaw(organ_data, fur_color))
}

pub fn dog_eye(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left eye" } else { "right eye" },
        BodyPartType::DogEye(organ_data),
    )
}

pub fn dog_ear(organ_data: OrganData, fur_color: FurColor, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left ear" } else { "right ear" },
        BodyPartType::DogEar(organ_data, fur_color),
    )
}

pub fn dog_head(organ_data: OrganData, fur_color: FurColor, sex: Sex) -> BodyPart {
    BodyPart::new(
        "head",
        BodyPartType::DogHead(organ_data.clone(), fur_color, sex),
    )
    .with_outside(vec![
        dog_eye(organ_data.clone(), true),
        dog_eye(organ_data.clone(), false),
        dog_ear(organ_data.clone(), fur_color, true),
        dog_ear(organ_data.clone(), fur_color, false),
        dog_nose(organ_data.clone(), fur_color),
        dog_maw(organ_data.clone(), fur_color),
    ])
    .with_inside(vec![dog_brain(organ_data)])
}

pub fn dog_paw(organ_data: OrganData, fur_color: FurColor, front: bool, left: bool) -> BodyPart {
    BodyPart::new(
        match (front, left) {
            (true, true) => "front left paw",
            (true, false) => "front right paw",
            (false, true) => "back left paw",
            (false, false) => "back right paw",
        },
        BodyPartType::DogPaw(organ_data, fur_color),
    )
}

pub fn dog_tail(organ_data: OrganData, fur_color: FurColor) -> BodyPart {
    BodyPart::new("tail", BodyPartType::DogTail(organ_data, fur_color))
}

pub fn dog_heart(organ_data: OrganData) -> BodyPart {
    BodyPart::new("heart", BodyPartType::DogHeart(organ_data))
}

pub fn dog_stomach(organ_data: OrganData) -> BodyPart {
    BodyPart::new("stomach", BodyPartType::DogStomach(organ_data))
}

pub fn dog_liver(organ_data: OrganData) -> BodyPart {
    BodyPart::new("liver", BodyPartType::DogLiver(organ_data))
}

pub fn dog_intestines(organ_data: OrganData) -> BodyPart {
    BodyPart::new("intestines", BodyPartType::DogIntestines(organ_data))
}

pub fn dog_lung(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left lung" } else { "right lung" },
        BodyPartType::DogLung(organ_data),
    )
}

pub fn dog_kidney(organ_data: OrganData, left: bool) -> BodyPart {
    BodyPart::new(
        if left { "left kidney" } else { "right kidney" },
        BodyPartType::DogKidney(organ_data),
    )
}

pub fn dog_torso(organ_data: OrganData, fur_color: FurColor, sex: Sex) -> BodyPart {
    BodyPart::new(
        "torso",
        BodyPartType::DogTorso(organ_data.clone(), fur_color, sex),
    )
    .with_outside(vec![
        dog_head(organ_data.clone(), fur_color, sex),
        dog_paw(organ_data.clone(), fur_color, true, true),
        dog_paw(organ_data.clone(), fur_color, true, false),
        dog_paw(organ_data.clone(), fur_color, false, true),
        dog_paw(organ_data.clone(), fur_color, false, false),
        dog_tail(organ_data.clone(), fur_color),
    ])
    .with_inside(vec![
        dog_heart(organ_data.clone()),
        dog_lung(organ_data.clone(), true),
        dog_lung(organ_data.clone(), false),
        dog_stomach(organ_data.clone()),
        dog_kidney(organ_data.clone(), true),
        dog_kidney(organ_data.clone(), false),
        dog_liver(organ_data.clone()),
        dog_intestines(organ_data),
    ])
}

pub fn dog_body(
    freshness: Freshness,
    age: u8,
    size: BodySize,
    fur_color: FurColor,
    sex: Sex,
) -> Body {
    let organ_data = OrganData {
        freshness,
        age,
        size,
        alive: true,
    };
    let parts = HashMap::from([(Point::new(0, 0), dog_torso(organ_data, fur_color, sex))]);
    Body::new(parts)
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use super::{
        super::super::{
            bodies::{BodySize, Freshness, OrganData, Sex},
            map::items::BodyPartType,
            ItemView,
        },
        dog_body, FurColor,
    };

    #[test]
    fn test_dog_body() {
        let body = dog_body(
            Freshness::Fresh,
            3,
            BodySize::Normal,
            FurColor::Black,
            Sex::Male,
        );
        let torso = body.parts.get(&Point::new(0, 0)).unwrap();
        assert_eq!("fresh dog torso", torso.name());
        assert!(matches!(
            torso.typ,
            BodyPartType::DogTorso(
                OrganData {
                    freshness: Freshness::Fresh,
                    age: 3,
                    size: BodySize::Normal,
                    alive: true,
                },
                FurColor::Black,
                Sex::Male,
            )
        ));
        let head = torso.outside.iter().next().unwrap();
        assert_eq!("fresh dog head", head.name());
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|i| matches!(i.typ, BodyPartType::DogEye(..)))
                .count()
        );
        assert_eq!(
            2,
            head.outside
                .iter()
                .filter(|i| matches!(i.typ, BodyPartType::DogEar(..)))
                .count()
        );
        assert_eq!(
            1,
            head.outside
                .iter()
                .filter(|i| matches!(i.typ, BodyPartType::DogMaw(..)))
                .count()
        );
        let brain = head.inside.iter().next().unwrap();
        assert_eq!("fresh dog brain", brain.name());
        assert!(matches!(
            brain.typ,
            BodyPartType::DogBrain(OrganData {
                freshness: Freshness::Fresh,
                age: 3,
                size: BodySize::Normal,
                alive: true,
            })
        ))
    }
}
