use geometry::Point;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use super::{
    super::{
        ai::AI,
        races::{Pronouns, Race},
        traits::LooksLike,
        Action, AttackType, BodySlot, CharSheet,
    },
    Appearance, Avatar, AvatarView, Fighter, Inventory, Weapon,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monster {
    id: usize,
    ai: AI,
    pos: Point,
    name: String,
    appearance: Appearance,
    pronouns: Pronouns,
    char_sheet: CharSheet,
    view: AvatarView,
    action: Option<Action>,
}

impl Monster {
    pub fn new(
        ai: AI,
        pos: Point,
        name: String,
        appearance: Appearance,
        pronouns: Pronouns,
        char_sheet: CharSheet,
    ) -> Self {
        Self {
            id: 0,
            view: AvatarView::new(
                appearance.race.looks_like(),
                appearance.body_color.map(Color::from),
            ),
            ai,
            pos,
            name,
            appearance,
            pronouns,
            char_sheet,
            action: None,
        }
    }
}

#[typetag::serde(name = "Monster")]
impl Avatar for Monster {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn ai(&self) -> Option<AI> {
        Some(self.ai)
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn appearance(&self) -> &Appearance {
        &self.appearance
    }

    fn pronouns(&self) -> Pronouns {
        self.pronouns
    }

    fn view(&self) -> &AvatarView {
        &self.view
    }

    fn view_mut(&mut self) -> &mut AvatarView {
        &mut self.view
    }

    fn char_sheet(&self) -> &CharSheet {
        &self.char_sheet
    }

    fn char_sheet_mut(&mut self) -> &mut CharSheet {
        &mut self.char_sheet
    }

    fn action(&self) -> Option<&Action> {
        self.action.as_ref()
    }

    fn set_action(&mut self, action: Option<Action>) {
        self.action = action;
    }

    fn as_monster(&self) -> Option<&Monster> {
        Some(self)
    }

    fn as_monster_mut(&mut self) -> Option<&mut Monster> {
        Some(self)
    }

    fn as_fighter(&self) -> &dyn Fighter {
        self
    }
}

impl Fighter for Monster {
    fn as_avatar(&self) -> &dyn Avatar {
        self
    }

    fn armor(&self, _slot: BodySlot) -> i8 {
        self.char_sheet.race.natural_armor()
    }

    fn weapon(&self, attack_type: AttackType) -> Option<Weapon> {
        // TODO: implement ranged natural weapons
        if attack_type == AttackType::Melee {
            Some(self.appearance.race.natural_weapon())
        } else {
            None
        }
    }
}
