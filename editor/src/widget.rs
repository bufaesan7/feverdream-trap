use std::fmt::Display;

use bevy::ecs::relationship::RelatedSpawnerCommands;

use crate::prelude::*;

/// A popup root UI node that fills the window and centers its content, blocking all UI below it.
pub fn popup_root(name: impl Display) -> impl Bundle {
    (
        Name::new(name.to_string()),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..default()
        },
        Transform::default(),
        BackgroundColor(Color::srgba(0., 0., 0., 0.7)),
    )
}

/// Helper root node for chaining items horizontally
pub fn ui_row() -> impl Bundle {
    (
        Name::new("Row"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            column_gap: px(5),
            ..default()
        },
        Transform::default(),
        Pickable::IGNORE,
    )
}

/// Helper root node for chaining items vertically
pub fn ui_col() -> impl Bundle {
    (
        Name::new("Column"),
        Node {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..default()
        },
        Transform::default(),
        Pickable::IGNORE,
    )
}

/// If `left_side` is false, the sidebar will be at the very right instead
pub fn sidebar(items_top: impl Bundle, items_bottom: impl Bundle, left_side: bool) -> impl Bundle {
    let mut root_node = Node {
        width: percent(30),
        height: percent(100),
        position_type: PositionType::Absolute,
        left: Val::ZERO,
        top: Val::ZERO,
        ..Default::default()
    };
    match left_side {
        true => root_node.left = Val::ZERO,
        false => root_node.right = Val::ZERO,
    }
    (
        Name::new("Sidebar"),
        root_node,
        Pickable::IGNORE,
        children![
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    ..Default::default()
                },
                Pickable::IGNORE,
                children![item_collection(items_top)],
            ),
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                Pickable::IGNORE,
                children![item_collection(items_bottom)],
            )
        ],
    )
}

/// A column of items
pub fn item_collection(items: impl Bundle) -> impl Bundle {
    (
        Name::new("Item collection"),
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            padding: UiRect::all(px(10)),
            ..Default::default()
        },
        Pickable::IGNORE,
        items,
    )
}

pub fn item_creation<S, M>(
    commands: &mut Commands,
    name: impl Display,
    input_bundle: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
    callback: S,
) -> Entity
where
    S: IntoSystem<(), (), M> + Send + 'static,
{
    let callback_id = commands.register_system(callback);
    let mut root = commands.spawn(editor_widget::popup_root(format!("{name} creation widget")));
    let root_id = root.id();
    root.with_child(widget::label_sized(format!("New {name}"), 12.));
    root.with_children(input_bundle);
    root.with_child(widget::button_small(
        "Create",
        move |_: On<Pointer<Click>>, mut commands: Commands| {
            commands.run_system(callback_id);
            commands.unregister_system(callback_id);
            commands.entity(root_id).despawn();
        },
    ));
    root_id
}

pub fn transform_editor() -> impl Bundle {
    (
        ui_col(),
        children![
            widget::label_sized("Transform", 12.),
            widget::label_sized("Position", 10.),
            (
                ui_row(),
                children![
                    numeric_input("x:", "1.0"),
                    numeric_input("y:", "1.0"),
                    numeric_input("z:", "1.0"),
                ]
            ),
            widget::label_sized("Scale", 10.),
            (
                ui_row(),
                children![
                    numeric_input("x:", "1.0"),
                    numeric_input("y:", "1.0"),
                    numeric_input("z:", "1.0"),
                ]
            ),
            widget::label_sized("EulerRot XYZ", 10.),
            (
                ui_row(),
                children![
                    numeric_input("x:", "0.0"),
                    numeric_input("y:", "0.0"),
                    numeric_input("z:", "0.0"),
                ]
            )
        ],
    )
}

fn numeric_input(label: impl Display, default_value: &'static str) -> impl Bundle {
    (
        ui_row(),
        children![
            widget::label_sized(label.to_string(), 10.),
            (
                TextEditable {
                    filter_in: vec!["[0-9]".into(), "\\.".into()],
                    ..Default::default()
                },
                Text::new(default_value),
                TextFont::from_font_size(10.)
            )
        ],
    )
}
