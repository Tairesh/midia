use std::cell::RefCell;

use geometry::{Point, Vec2};
use tetra::graphics::{Canvas, DrawParams};
use tetra::Context;

use crate::assets::{Assets, Sprite, Tileset};
use crate::colors::Colors;
use crate::game::map::TerrainView;
use crate::game::traits::LooksLike;
use crate::game::{Avatar, Tile, World};
use crate::scenes::game_modes::Cursor;

// TODO: refactor this shit

#[allow(clippy::too_many_lines)]
pub fn draw(
    ctx: &mut Context,
    world: &RefCell<World>,
    assets: &Assets,
    window_size: (i32, i32),
    shift_of_view: Point,
) -> Canvas {
    let canvas = Canvas::new(ctx, window_size.0, window_size.1).unwrap();
    tetra::graphics::set_canvas(ctx, &canvas);
    tetra::graphics::clear(ctx, Colors::BLACK);
    let world = world.borrow();

    let scale = world.game_view.zoom.as_scale();
    let zoom = world.game_view.zoom.as_view();
    let tile_size = assets.tileset.tile_size as f32 * zoom;

    let (width, height) = (
        window_size.0 / tile_size as i32,
        window_size.1 / tile_size as i32,
    );
    let center = Vec2::new(window_size.0 as f32, window_size.1 as f32) / 2.0
        - Vec2::new(tile_size, tile_size) / 2.0;
    let center_tile = world.units().player().pos + shift_of_view;

    let left_top = center_tile + (-width / 2, -height / 2);
    let right_bottom = center_tile + (width / 2, height / 2);

    let mut map = world.map();
    map.load_tiles_between(left_top, right_bottom);

    // TODO: is_visible takes like 12% of processing time
    let tiles = map
        .tiles_between(left_top, right_bottom)
        .into_iter()
        .filter(|(pos, _)| world.is_visible(*pos))
        .collect::<Vec<(Point, &Tile)>>();
    // TODO: looks like x and y are swapped
    for &(pos, tile) in &tiles {
        let delta = Vec2::from(pos - center_tile);
        let position = center + delta * tile_size;

        let this_tile_size = Tileset::get_size(tile.terrain.looks_like());
        let asset_tile_size = Vec2::new(
            assets.tileset.tile_size as f32,
            assets.tileset.tile_size as f32,
        );
        if this_tile_size == asset_tile_size {
            assets.tileset.draw_sprite(
                ctx,
                tile.terrain.looks_like(),
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(tile.terrain.color().unwrap_or(Colors::WHITE)),
            );
        }
        if let Some(item) = tile.top_item() {
            let this_tile_size = Tileset::get_size(item.looks_like());
            let mut correction = -(this_tile_size - asset_tile_size) * zoom;
            correction.x /= 2.0;

            assets.tileset.draw_sprite(
                ctx,
                item.looks_like(),
                DrawParams::new()
                    .position(position + correction)
                    .scale(scale)
                    .color(item.color()),
            );
            if tile.items.len() > 1 {
                assets.tileset.draw_sprite(
                    ctx,
                    Sprite::Highlight,
                    DrawParams::new().position(position).scale(scale),
                );
            }
        }
    }
    for &(pos, tile) in &tiles {
        let position = center + Vec2::from(pos - center_tile) * tile_size;

        for i in tile.units.iter().copied() {
            draw_unit(
                ctx,
                &assets.tileset,
                position,
                zoom,
                true,
                world.units().get_unit(i).as_fighter().as_avatar(),
            );
            if !tile.items.is_empty() {
                assets.tileset.draw_sprite(
                    ctx,
                    Sprite::Highlight,
                    DrawParams::new().position(position).scale(scale),
                );
            }
        }
    }
    for &(pos, tile) in &tiles {
        let position = center + Vec2::from(pos - center_tile) * tile_size;

        let this_tile_size = Tileset::get_size(tile.terrain.looks_like());
        let asset_tile_size = Vec2::new(
            assets.tileset.tile_size as f32,
            assets.tileset.tile_size as f32,
        );
        if this_tile_size != asset_tile_size {
            let mut correction = -(this_tile_size - asset_tile_size) * zoom;
            correction.x /= 2.0;

            assets.tileset.draw_sprite(
                ctx,
                tile.terrain.looks_like(),
                DrawParams::new()
                    .position(position + correction)
                    .scale(scale)
                    .color(tile.terrain.color().unwrap_or(Colors::WHITE)),
            );
        }
    }

    // if world.player().action.is_some() {
    //     self.draw_action_loader(ctx, center);
    // } else {
    //     self.action_text = None;
    // }

    tetra::graphics::reset_canvas(ctx);
    canvas
}

pub fn draw_cursors(
    ctx: &mut Context,
    world: &RefCell<World>,
    assets: &Assets,
    window_size: (i32, i32),
    cursors: Vec<Cursor>,
) {
    let world = world.borrow();

    let scale = world.game_view.zoom.as_scale();
    let zoom = world.game_view.zoom.as_view();
    let tile_size = assets.tileset.tile_size as f32 * zoom;
    let center = Vec2::new(window_size.0 as f32, window_size.1 as f32) / 2.0
        - Vec2::new(tile_size, tile_size) / 2.0;

    for (delta, color, typ) in cursors {
        let delta = delta * tile_size;
        let position = center + delta;

        let params = DrawParams::new()
            .position(position)
            .scale(scale)
            .color(color);
        assets.tileset.draw_sprite(ctx, typ.looks_like(), params);
    }
}

pub fn draw_unit(
    ctx: &mut Context,
    tileset: &Tileset,
    mut position: Vec2,
    zoom: f32,
    rotate: bool,
    avatar: &dyn Avatar,
) {
    let scale = if !rotate || avatar.view().direction().is_default() {
        Vec2::new(zoom, zoom)
    } else {
        position.x += 10.0 * zoom;
        Vec2::new(-zoom, zoom)
    };
    let mut draw_params = DrawParams::new().position(position).scale(scale);
    if let Some(color) = avatar.view().fg() {
        draw_params = draw_params.color(color);
    }
    tileset.draw_sprite(ctx, avatar.view().looks_like(), draw_params);

    // TODO: draw both items
    if avatar.inventory().is_none() || avatar.inventory().unwrap().main_hand().is_none() {
        return;
    }
    let item = avatar.inventory().unwrap().main_hand().unwrap();
    let (offset_x, offset_y) = (
        if !rotate || avatar.view().direction().is_default() {
            5.0
        } else {
            -5.0
        } * zoom,
        3.0 * zoom,
    );
    tileset.draw_sprite(
        ctx,
        item.looks_like(),
        DrawParams::new()
            .position(position + Vec2::new(offset_x, offset_y))
            .color(item.color())
            .scale(scale * 0.7),
    );
}
