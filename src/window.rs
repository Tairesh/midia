use tetra::{
    graphics::ImageData,
    window::{self, WindowPosition::Centered},
    Context, ContextBuilder, Result,
};

use crate::settings::Settings;

pub fn create_window(title: impl Into<String>) -> Result<Context> {
    let window_settings = &Settings::instance().window;

    let mut ctx = ContextBuilder::new(title, window_settings.width, window_settings.height)
        .show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true)
        .build()?;

    let mut icon = ImageData::from_encoded(include_bytes!("../assets/img/icon.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;

    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;

    if window_settings.fullscreen {
        window::set_fullscreen(&mut ctx, true).ok();
    } else {
        let monitor = window::get_current_monitor(&ctx).unwrap_or(0) as i32;
        window::set_position(&mut ctx, Centered(monitor), Centered(monitor));
    }

    Ok(ctx)
}
