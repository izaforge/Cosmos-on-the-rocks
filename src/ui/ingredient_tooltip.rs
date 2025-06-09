// use crate::{
//     bar::{crafting::OnCraftingScreen, glass::Glass, ingredient::Ingredient},
//     constants::TEXT_COLOR,
// };
// use bevy::{
//     picking::events::{Out, Over, Pointer},
//     prelude::*,
// };

// /// Component marker for ingredient tooltip UI
// #[derive(Component)]
// pub struct IngredientTooltip;

// /// Component marker for glass tooltip UI
// #[derive(Component)]
// pub struct GlassTooltip;

// /// Resource to track which ingredient is currently being hovered
// #[derive(Resource, Default)]
// pub struct HoveredIngredient {
//     pub entity: Option<Entity>,
//     pub position: Vec2,
// }

// /// System to setup tooltips for ingredients - called when ingredients are spawned
// pub fn setup_ingredient_tooltips(
//     mut commands: Commands,
//     ingredient_query: Query<Entity, (With<Ingredient>, Without<IngredientTooltip>)>,
// ) {
//     for entity in ingredient_query.iter() {
//         commands
//             .entity(entity)
//             .observe(show_tooltip_on_hover)
//             .observe(hide_tooltip_on_exit);
//     }
// }

// /// System to setup tooltips for glasses - called when glasses are spawned
// pub fn setup_glass_tooltips(
//     mut commands: Commands,
//     glass_query: Query<Entity, (With<Glass>, Without<GlassTooltip>)>,
// ) {
//     for entity in glass_query.iter() {
//         commands
//             .entity(entity)
//             .observe(show_glass_tooltip_on_hover)
//             .observe(hide_glass_tooltip_on_exit);
//     }
// }

// /// Observer function to show tooltip when hovering over an ingredient
// fn show_tooltip_on_hover(
//     trigger: Trigger<Pointer<Over>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     ingredient_query: Query<(&GlobalTransform, &Ingredient)>,
//     tooltip_query: Query<Entity, With<IngredientTooltip>>,
//     mut hovered_ingredient: ResMut<HoveredIngredient>,
//     windows: Query<&Window>,
// ) {
//     let Ok(window) = windows.single() else {
//         return;
//     };
//     let Ok((transform, ingredient)) = ingredient_query.get(trigger.target()) else {
//         return;
//     };

//     // Remove any existing tooltip
//     for tooltip_entity in tooltip_query.iter() {
//         commands.entity(tooltip_entity).despawn();
//     }

//     // Update hovered ingredient
//     let world_pos = transform.translation().truncate();
//     hovered_ingredient.entity = Some(trigger.target());
//     hovered_ingredient.position = world_pos;

//     let font = asset_server.load("fonts/Nasa21.ttf");

//     // Create tooltip
//     let tooltip_text = format!(
//         "{}\n{}\nSize: {:.1}\nTaste: {:?}\nEffect: {:?}",
//         ingredient.name,
//         ingredient.description,
//         ingredient.ingredient_profile.size,
//         ingredient.ingredient_profile.taste,
//         ingredient.ingredient_profile.primary_effect
//     );

//     commands
//         .spawn((
//             IngredientTooltip,
//             OnCraftingScreen,
//             Node {
//                 position_type: PositionType::Absolute,
//                 left: Val::Px(world_pos.x + 100.0),
//                 top: Val::Px(window.height() - world_pos.y - 100.0),
//                 padding: UiRect::all(Val::Px(10.0)),
//                 flex_direction: FlexDirection::Column,
//                 max_width: Val::Px(250.0),
//                 ..default()
//             },
//             BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
//             BorderRadius::all(Val::Px(5.0)),
//             ZIndex(1000),
//         ))
//         .with_children(|parent| {
//             parent.spawn((
//                 Text::new(tooltip_text),
//                 TextFont {
//                     font: font.clone(),
//                     font_size: 14.0,
//                     ..default()
//                 },
//                 TextColor(TEXT_COLOR),
//             ));
//         });
// }

// /// Observer function to hide tooltip when exiting hover
// fn hide_tooltip_on_exit(
//     trigger: Trigger<Pointer<Out>>,
//     mut commands: Commands,
//     tooltip_query: Query<Entity, With<IngredientTooltip>>,
//     mut hovered_ingredient: ResMut<HoveredIngredient>,
// ) {
//     // Only process if this is the currently hovered ingredient
//     if hovered_ingredient.entity == Some(trigger.target()) {
//         // Remove tooltip when no longer hovering
//         for tooltip_entity in tooltip_query.iter() {
//             commands.entity(tooltip_entity).despawn();
//         }
//         hovered_ingredient.entity = None;
//     }
// }

// /// System to cleanup glass tooltips when exiting crafting state
// pub fn cleanup_glass_tooltips(
//     mut commands: Commands,
//     tooltip_query: Query<Entity, With<GlassTooltip>>,
// ) {
//     for tooltip_entity in tooltip_query.iter() {
//         commands.entity(tooltip_entity).despawn();
//     }
// }

// /// Observer function to show tooltip when hovering over a glass
// fn show_glass_tooltip_on_hover(
//     trigger: Trigger<Pointer<Over>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     glass_query: Query<(&GlobalTransform, &Glass)>,
//     tooltip_query: Query<Entity, With<GlassTooltip>>,
//     windows: Query<&Window>,
// ) {
//     let Ok(window) = windows.single() else {
//         return;
//     };
//     let Ok((transform, glass)) = glass_query.get(trigger.target()) else {
//         return;
//     };

//     // Remove any existing glass tooltip
//     for tooltip_entity in tooltip_query.iter() {
//         commands.entity(tooltip_entity).despawn();
//     }

//     let world_pos = transform.translation().truncate();
//     let font = asset_server.load("fonts/Nasa21.ttf");

//     // Create glass tooltip
//     let current_volume = glass.get_current_volume();
//     let tooltip_text = format!(
//         "Glass\nCapacity: {:.1}\nCurrent Volume: {:.1}\nIngredients: {}",
//         glass.capacity,
//         current_volume,
//         glass.ingredients.len()
//     );

//     commands
//         .spawn((
//             GlassTooltip,
//             OnCraftingScreen,
//             Node {
//                 position_type: PositionType::Absolute,
//                 left: Val::Px(world_pos.x + 100.0),
//                 top: Val::Px(window.height() - world_pos.y - 100.0),
//                 padding: UiRect::all(Val::Px(10.0)),
//                 flex_direction: FlexDirection::Column,
//                 max_width: Val::Px(200.0),
//                 ..default()
//             },
//             BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
//             BorderRadius::all(Val::Px(5.0)),
//             ZIndex(1000),
//         ))
//         .with_children(|parent| {
//             parent.spawn((
//                 Text::new(tooltip_text),
//                 TextFont {
//                     font: font.clone(),
//                     font_size: 14.0,
//                     ..default()
//                 },
//                 TextColor(TEXT_COLOR),
//             ));
//         });
// }

// /// Observer function to hide glass tooltip when exiting hover
// fn hide_glass_tooltip_on_exit(
//     _trigger: Trigger<Pointer<Out>>,
//     mut commands: Commands,
//     tooltip_query: Query<Entity, With<GlassTooltip>>,
// ) {
//     // Remove glass tooltip when no longer hovering
//     for tooltip_entity in tooltip_query.iter() {
//         commands.entity(tooltip_entity).despawn();
//     }
// }
