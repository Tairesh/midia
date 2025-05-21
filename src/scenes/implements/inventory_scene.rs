use std::cell::RefCell;
use std::rc::Rc;
use tetra::{input::Key, Context};

use crate::{
    app::App,
    assets::Assets,
    colors::Colors,
    game::{Item, World},
    input,
    scenes::{SceneImpl, Transition},
    ui::{
        Alert, ButtonBuilder, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut,
        TilesetSprite, UiSprite, Vertical,
    },
};

pub struct InventoryScene {
    sprites: Vec<Box<dyn UiSprite>>,
}

impl InventoryScene {
    pub fn new(app: &App, _ctx: &mut Context) -> Self {
        let mut sprites: Vec<Box<dyn UiSprite>> = Vec::new();

        let alert = Box::new(Alert::new(
            500.0,
            400.0,
            app.assets.alert.clone(),
            Position::center(),
        ));
        sprites.push(alert);

        let world = app.get_world();
        let world_borrow = world.borrow();
        let player = world_borrow.units().player();

        let mut current_y_offset = 20.0; // Initial Y for titles

        // Wielded Title
        let wielded_title_pos = Position {
            x: Horizontal::AtWindowCenterByLeft { offset: -230.0 },
            y: Vertical::ByTop {
                y: current_y_offset,
            },
        };
        let wielded_title = Box::new(Label::new(
            "Wielded:",
            app.assets.fonts.header.clone(),
            Colors::WHITE_SMOKE,
            wielded_title_pos,
        ));
        sprites.push(wielded_title);
        current_y_offset += 40.0; // Space after title

        // Wielded Items Loop
        let item_x_icon = Horizontal::AtWindowCenterByLeft { offset: -230.0 };
        let item_x_label = Horizontal::AtWindowCenterByLeft { offset: -200.0 };
        for item in player.inventory.wield.items.iter() {
            let icon_pos = Position {
                x: item_x_icon,
                y: Vertical::ByTop {
                    y: current_y_offset,
                },
            };
            let icon = Box::new(TilesetSprite::new(
                item.looks_like(),
                app.assets.tileset.clone(),
                icon_pos,
                1.0, // Default scale
                item.color(),
            ));
            sprites.push(icon);

            let label_pos = Position {
                x: item_x_label,
                y: Vertical::ByTop {
                    y: current_y_offset,
                },
            };
            let name_label = Box::new(Label::new(
                item.name(),
                app.assets.fonts.default.clone(),
                Colors::WHITE_SMOKE,
                label_pos,
            ));
            sprites.push(name_label);
            current_y_offset += 25.0;
        }

        current_y_offset += 20.0; // Padding before next section

        // Worn Title
        let worn_title_pos = Position {
            x: Horizontal::AtWindowCenterByLeft { offset: -230.0 },
            y: Vertical::ByTop {
                y: current_y_offset,
            },
        };
        let worn_title = Box::new(Label::new(
            "Worn:",
            app.assets.fonts.header.clone(),
            Colors::WHITE_SMOKE,
            worn_title_pos,
        ));
        sprites.push(worn_title);
        current_y_offset += 40.0;

        // Worn Items Loop (and inner container loop)
        let worn_item_x_icon = Horizontal::AtWindowCenterByLeft { offset: -230.0 };
        let worn_item_x_label = Horizontal::AtWindowCenterByLeft { offset: -200.0 };
        let inner_item_x_icon = Horizontal::AtWindowCenterByLeft { offset: -210.0 }; // Indented
        let inner_item_x_label = Horizontal::AtWindowCenterByLeft { offset: -180.0 }; // Indented

        for item in player.inventory.wear.iter() {
            if let Some(item) = item {
                // Item in wear slot
                let icon_pos = Position {
                    x: worn_item_x_icon,
                    y: Vertical::ByTop {
                        y: current_y_offset,
                    },
                };
                let icon = Box::new(TilesetSprite::new(
                    item.looks_like(),
                    app.assets.tileset.clone(),
                    icon_pos,
                    1.0,
                    item.color(),
                ));
                sprites.push(icon);

                let label_pos = Position {
                    x: worn_item_x_label,
                    y: Vertical::ByTop {
                        y: current_y_offset,
                    },
                };
                let name_label = Box::new(Label::new(
                    item.name(),
                    app.assets.fonts.default.clone(),
                    Colors::WHITE_SMOKE,
                    label_pos,
                ));
                sprites.push(name_label);
                current_y_offset += 25.0;

                if let Some(container) = item.container() {
                    for inner_item in container.items.iter() {
                        let inner_icon_pos = Position {
                            x: inner_item_x_icon,
                            y: Vertical::ByTop {
                                y: current_y_offset,
                            },
                        };
                        let inner_icon = Box::new(TilesetSprite::new(
                            inner_item.looks_like(),
                            app.assets.tileset.clone(),
                            inner_icon_pos,
                            1.0,
                            inner_item.color(),
                        ));
                        sprites.push(inner_icon);

                        let inner_label_pos = Position {
                            x: inner_item_x_label,
                            y: Vertical::ByTop {
                                y: current_y_offset,
                            },
                        };
                        let inner_name_label = Box::new(Label::new(
                            format!("- {}", inner_item.name()),
                            app.assets.fonts.default.clone(),
                            Colors::WHITE_SMOKE,
                            inner_label_pos,
                        ));
                        sprites.push(inner_name_label);
                        current_y_offset += 25.0;
                    }
                }
            }
        }

        drop(player);
        drop(world_borrow);

        let back_btn = ButtonBuilder::new()
            .with_text("Back ([Esc])")
            .with_position(Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByBottom { y: -30.0 },
            })
            .with_keys(vec![Key::Escape.into()])
            .with_action(Transition::Pop)
            .build(app.assets.button.clone());
        sprites.push(Box::new(back_btn));

        Self { sprites }
    }
}

impl SceneImpl for InventoryScene {
    fn on_update(&mut self, _ctx: &mut Context, _app: &App) -> Option<Transition> {
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: input::Event, _app: &App) -> Option<Transition> {
        if let input::Event::KeyboardKeyReleased(key) = event {
            if key == Key::Escape || key == Key::I {
                return Some(Transition::Pop);
            }
        }
        None
    }

    fn before_draw(&mut self, _ctx: &mut Context, _app: &App) {
        // Optional: Could clear the screen or draw a background
    }

    fn after_draw(&mut self, _ctx: &mut Context, _app: &App) {
        // Optional: Could draw UI elements on top of everything
    }

    fn on_open(&mut self, _ctx: &mut Context, _app: &App) {
        // Optional: Initialize scene state
    }

    fn on_resize(&mut self, _ctx: &mut Context, _app: &App) {
        // Optional: Adjust layout based on new screen size
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, _event: u32, _param: u32) -> Option<Transition> {
        None
    }
}
