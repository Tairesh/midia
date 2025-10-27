use super::{
    super::{
        races::Pronouns,
        traits::{LooksLike, Name},
        Action, AttackType, BodySlot, CharSheet,
    },
    Appearance, Avatar, AvatarView, Fighter, Inventory, PlayerPersonality, Weapon,
};
use roguemetry::Point;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub pos: Point,
    pub personality: PlayerPersonality,
    pub view: AvatarView,
    pub inventory: Inventory,
    pub action: Option<Action>,
}

impl Player {
    pub fn new(personality: PlayerPersonality, pos: Point) -> Self {
        Self {
            id: 0,
            view: AvatarView::new(
                personality.appearance.race.looks_like(),
                personality.appearance.body_color.map(Color::from),
            ),
            inventory: Inventory::humanoid(),
            personality,
            pos,
            action: None,
        }
    }
}

#[typetag::serde(name = "Player")]
impl Avatar for Player {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn pos(&self) -> Point {
        self.pos
    }

    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn name(&self) -> &str {
        &self.personality.mind.name
    }

    fn name_for_actions(&self) -> &'static str {
        "you"
    }

    fn appearance(&self) -> &Appearance {
        &self.personality.appearance
    }

    fn is_player(&self) -> bool {
        true
    }

    fn pronouns(&self) -> Pronouns {
        Pronouns::YouYour
    }

    fn view(&self) -> &AvatarView {
        &self.view
    }

    fn view_mut(&mut self) -> &mut AvatarView {
        &mut self.view
    }

    fn char_sheet(&self) -> &CharSheet {
        &self.personality.char_sheet
    }

    fn char_sheet_mut(&mut self) -> &mut CharSheet {
        &mut self.personality.char_sheet
    }

    fn action(&self) -> Option<&Action> {
        self.action.as_ref()
    }

    fn set_action(&mut self, action: Option<Action>) {
        self.action = action;
    }

    fn inventory(&self) -> Option<&Inventory> {
        Some(&self.inventory)
    }

    fn inventory_mut(&mut self) -> Option<&mut Inventory> {
        Some(&mut self.inventory)
    }

    fn as_player(&self) -> Option<&Player> {
        Some(self)
    }

    fn as_player_mut(&mut self) -> Option<&mut Player> {
        Some(self)
    }

    fn as_fighter(&self) -> &dyn Fighter {
        self
    }
}

impl Fighter for Player {
    fn as_avatar(&self) -> &dyn Avatar {
        self
    }

    // TODO: trait bonuses for armor, toughness, parry
    fn armor(&self, slot: BodySlot) -> i8 {
        self.inventory().map_or(0, |i| {
            i.get_items_by_slot(slot)
                .into_iter()
                .map(|item| item.armor() as i8)
                .sum::<i8>()
        }) + self.personality.char_sheet.race.natural_armor()
    }

    fn weapon(&self, attack_type: AttackType) -> Option<Weapon> {
        // TODO: refactor with Weapon::from_item
        if let Some(weapon) = self.inventory.main_hand() {
            if attack_type == AttackType::Melee || weapon.need_ammo().is_none() {
                return weapon.damage(attack_type).map(|damage| Weapon {
                    name: weapon.name(),
                    ammo_name: None,
                    damage,
                });
            }

            if let Some(ammo) = weapon.container().unwrap().items.first() {
                if let Some(ammo_value) = &ammo.proto().is_ammo {
                    let mut damage = weapon.damage(attack_type).unwrap();
                    damage.damage.modifier += ammo_value.damage_modifier.damage;
                    damage.penetration += ammo_value.damage_modifier.penetration;
                    if let Some(dice) = ammo_value.damage_modifier.damage_dice {
                        damage.damage.dices.push(dice);
                    }
                    return Some(Weapon {
                        name: weapon.name(),
                        ammo_name: Some(ammo.name()),
                        damage,
                    });
                }
            }

            return weapon.damage(attack_type).map(|damage| Weapon {
                name: weapon.name(),
                ammo_name: None,
                damage,
            });
        }

        // TODO: natural range weapons
        if attack_type == AttackType::Melee {
            Some(self.personality.appearance.race.natural_weapon())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::helpers::tester_girl, *};

    #[test]
    fn test_natural_weapon() {
        let mut player = Player::new(tester_girl(), Point::new(0, 0));
        player.inventory.clear();
        assert!(player.weapon(AttackType::Melee).is_some());
        let weapon = player.weapon(AttackType::Melee).unwrap();
        assert_eq!(weapon.name, "fists");
    }

    #[test]
    fn test_weapon_is_none_to_throw() {
        let mut player = Player::new(tester_girl(), Point::new(0, 0));
        player.inventory.clear();
        assert!(player.weapon(AttackType::Throw).is_none());
    }
}
