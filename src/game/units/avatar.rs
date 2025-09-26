use roguemetry::{Direction, Point, TwoDimDirection};
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use super::{
    super::{
        super::assets::Sprite, ai::AI, map::items::helpers::dead_body, races::Pronouns,
        savage::HitResult, traits::LooksLike, Action, AttackType, BodySlot, CharSheet, Item,
    },
    Appearance, Fighter, Inventory, Monster, Player, Weapon,
};

#[typetag::serde(tag = "Avatar")]
pub trait Avatar {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn ai(&self) -> Option<AI> {
        None
    }
    fn pos(&self) -> Point;
    fn set_pos(&mut self, pos: Point);
    fn name(&self) -> &str;
    fn name_for_actions(&self) -> &str {
        self.name()
    }
    fn appearance(&self) -> &Appearance;
    fn is_player(&self) -> bool {
        false
    }
    fn pronouns(&self) -> Pronouns;
    fn view(&self) -> &AvatarView;
    fn view_mut(&mut self) -> &mut AvatarView;
    fn char_sheet(&self) -> &CharSheet;
    fn char_sheet_mut(&mut self) -> &mut CharSheet;
    fn action(&self) -> Option<&Action>;
    fn set_action(&mut self, action: Option<Action>);
    fn inventory(&self) -> Option<&Inventory> {
        None
    }
    fn inventory_mut(&mut self) -> Option<&mut Inventory> {
        None
    }
    fn as_player(&self) -> Option<&Player> {
        None
    }
    fn as_player_mut(&mut self) -> Option<&mut Player> {
        None
    }
    fn as_monster(&self) -> Option<&Monster> {
        None
    }
    fn as_monster_mut(&mut self) -> Option<&mut Monster> {
        None
    }
    fn as_fighter(&self) -> &dyn Fighter;
    fn apply_hit(&mut self, hit: HitResult, current_tick: u128) -> Option<Vec<Item>> {
        self.char_sheet_mut().apply_hit(hit, current_tick);

        // TODO: drop weapons if arm is wounded

        if self.char_sheet().is_dead() {
            self.set_action(None);

            let mut items = Vec::new();
            if let Some(inventory) = self.inventory_mut() {
                items.append(&mut inventory.take_all());
            }
            items.push(dead_body(self.appearance()));
            return Some(items);
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarView {
    direction: TwoDimDirection,
    sprite: Sprite,
    fg: Option<Color>,
}

impl AvatarView {
    pub fn new(sprite: Sprite, fg: Option<Color>) -> Self {
        Self {
            direction: TwoDimDirection::default(),
            sprite,
            fg,
        }
    }

    pub fn direction(&self) -> TwoDimDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: TwoDimDirection) {
        self.direction = direction;
    }

    pub fn try_set_direction(&mut self, direction: Direction) -> bool {
        if let Ok(direction) = direction.try_into() {
            self.direction = direction;
            return true;
        }

        false
    }

    pub fn fg(&self) -> Option<Color> {
        self.fg
    }
}

impl LooksLike for AvatarView {
    fn looks_like(&self) -> Sprite {
        self.sprite
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::Point;

    use crate::game::map::items::helpers::{CLOAK, GOD_AXE};
    use crate::game::traits::Name;

    use super::{super::tests::helpers::tester_girl, *};

    #[test]
    fn test_player_name() {
        let mut player = Player::new(tester_girl(), Point::new(0, 0));
        player.id = 0;

        assert_eq!(player.name_for_actions(), "you");
    }

    // TODO: test_npc_name

    #[test]
    fn test_die() {
        let mut avatar = Player::new(tester_girl(), Point::new(0, 0));
        avatar.inventory_mut().unwrap().wield(Item::new(GOD_AXE));
        avatar.inventory_mut().unwrap().wear(Item::new(CLOAK), 0);
        let items = avatar.apply_hit(HitResult::ultra_damage(), 0);
        assert!(items.is_some());
        let items = items.unwrap();
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|i| i.proto().id == CLOAK));
        assert!(items.iter().any(|i| i.proto().id == GOD_AXE));
        assert_eq!(items[2].name(), "dead gazan girl");
        assert!(avatar.char_sheet().is_dead());
    }
}
