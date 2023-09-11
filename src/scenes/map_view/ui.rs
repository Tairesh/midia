use geometry::Vec2;
use tetra::Context;

use crate::ui::GameLog;

pub fn draw_log(ctx: &mut Context, log: &mut GameLog) {
    let window_size = tetra::window::get_size(ctx);
    log.texts.iter_mut().enumerate().for_each(|(i, msg)| {
        msg.draw(
            Vec2::new(10.0, window_size.1 as f32 - 20.0 * (i + 1) as f32),
            ctx,
        );
    });
}
