use bevy::{prelude::*, sprite::Anchor, window::SystemCursorIcon};
use bevy_lunex::*;

use crate::{
    bar::crafting::OnCraftingScreen,
    constants::{NORMAL_BUTTON, TEXT_COLOR},
};

pub fn spawn_craft_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");
    commands
        .spawn((
            OnCraftingScreen,
            Name::new("CraftUI"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<1>,
        ))
        .with_children(|ui| {
            // Craft Button
            ui.spawn((
                Name::new("Craft Button Container"),
                UiLayout::window()
                    .pos(Rl((40.0, 20.0))) // 40% from left, 20% from bottom
                    .anchor(Anchor::BottomCenter) // Anchor to bottom center
                    .size(Rl((15.0, 6.0))) // 15% width, 6% height of screen
                    .pack(),
            ))
            .with_children(|craft_btn| {
                craft_btn
                    .spawn((
                        Name::new("Craft Button Background"),
                        UiLayout::solid().size((1.0, 1.0)).pack(), // Fill container
                        Sprite {
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                    ))
                    .observe(|_: Trigger<Pointer<Click>>| info!("Craft button clicked!"));

                craft_btn.spawn((
                    Name::new("Craft Button Text"),
                    UiLayout::window().anchor(Anchor::Center).pack(),
                    Text::new("CRAFT"),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR.into()),
                ));
            });

            // Reset Button
            ui.spawn((
                Name::new("Reset Button Container"),
                UiLayout::window()
                    .pos(Rl((60.0, 20.0))) // 60% from left, 20% from bottom
                    .anchor(Anchor::BottomCenter) // Anchor to bottom center
                    .size(Rl((15.0, 6.0))) // 15% width, 6% height of screen
                    .pack(),
            ))
            .with_children(|reset_btn| {
                reset_btn
                    .spawn((
                        Name::new("Reset Button Background"),
                        UiLayout::solid().size((1.0, 1.0)).pack(), // Fill container
                        Sprite {
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                    ))
                    .observe(|_: Trigger<Pointer<Click>>| info!("Reset button clicked!"));

                reset_btn.spawn((
                    Name::new("Reset Button Text"),
                    UiLayout::window().anchor(Anchor::Center).pack(),
                    Text::new("RESET"),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR.into()),
                ));
            });
        });
}
