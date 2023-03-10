use crate::game::savage::HitResult;
use crate::game::Avatar;

use super::{LogCategory, LogEvent};

pub fn unit_attack_success(
    owner: &Avatar,
    unit: &Avatar,
    damage: &HitResult,
    first_message: String,
) -> Vec<LogEvent> {
    let mut events = Vec::new();

    events.push(LogEvent::new(
        first_message,
        unit.pos,
        if owner.is_player() {
            LogCategory::Success
        } else {
            LogCategory::Danger
        },
    ));
    if damage.params.critical {
        events.push(LogEvent::new(
            "Critical hit!".to_string(),
            unit.pos,
            if owner.is_player() {
                LogCategory::Success
            } else {
                LogCategory::Danger
            },
        ));
    }
    if damage.causes.shock {
        if damage.causes.wounds.is_empty() {
            events.push(LogEvent::new(
                format!("{} is stunned.", unit.name_for_actions()),
                unit.pos,
                if owner.is_player() {
                    LogCategory::Success
                } else {
                    LogCategory::Danger
                },
            ));
        } else {
            events.push(LogEvent::new(
                format!("{} is stunned and wounded.", unit.name_for_actions()),
                unit.pos,
                if owner.is_player() {
                    LogCategory::Success
                } else {
                    LogCategory::Danger
                },
            ));
        }
    }

    events.push(LogEvent::debug(
        format!(
            "Damage: {}, penetration: {}, crit: {:?}, shock: {:?}, wounds: {:?}",
            damage.params.damage,
            damage.params.penetration,
            damage.params.critical,
            damage.causes.shock,
            damage.causes.wounds
        ),
        unit.pos,
    ));

    events
}
