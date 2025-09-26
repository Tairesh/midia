use crate::game::{savage::HitResult, Avatar};
use roguemetry::Point;

use super::{LogCategory, LogEvent};

pub fn unit_attack_success(
    owner: &dyn Avatar,
    unit: &dyn Avatar,
    hit: &HitResult,
    first_message: String,
) -> Vec<LogEvent> {
    let mut events = Vec::new();

    push_event(&mut events, first_message, owner, unit);
    if hit.params.critical {
        push_event(&mut events, "Critical hit!".to_string(), owner, unit);
    }
    if hit.consequences.shock {
        if hit.consequences.wounds.is_empty() {
            push_event(
                &mut events,
                format!(
                    "{} {} stunned.",
                    unit.name_for_actions(),
                    unit.pronouns().is_are()
                ),
                owner,
                unit,
            );
        } else {
            push_event(
                &mut events,
                format!(
                    "{} {} stunned and wounded.",
                    unit.name_for_actions(),
                    unit.pronouns().is_are()
                ),
                owner,
                unit,
            );
        }
    }

    events.push(LogEvent::debug(
        format!(
            "Damage: {}, penetration: {}, crit: {:?}, shock: {:?}, wounds: {:?}",
            hit.params.damage,
            hit.params.penetration,
            hit.params.critical,
            hit.consequences.shock,
            hit.consequences.wounds
        ),
        unit.pos(),
    ));

    events
}

fn push_event(events: &mut Vec<LogEvent>, message: String, owner: &dyn Avatar, unit: &dyn Avatar) {
    events.push(LogEvent::new(
        message,
        unit.pos(),
        if owner.is_player() {
            LogCategory::Success
        } else {
            LogCategory::Danger
        },
    ));
}
