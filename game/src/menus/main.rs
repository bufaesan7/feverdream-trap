//! The main menu (seen on the title screen).

use crate::prelude::*;

/// Marker indicating the menu buttons have already been shuffled as a prank.
#[derive(Component)]
struct MenuPranked;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_observer(rotate_buttons_on_hover);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

/// On any button hover, rotate the main menu buttons once to the right. A prank.
fn rotate_buttons_on_hover(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    buttons: Query<(), With<Button>>,
    menu_root_query: Query<(Entity, &Children), (With<DespawnOnExit<Menu>>, Without<MenuPranked>)>,
) {
    if buttons.get(over.event_target()).is_err() {
        return;
    }

    for (root_entity, children) in &menu_root_query {
        let mut order: Vec<Entity> = children.iter().collect();
        if order.len() < 2 {
            continue;
        }
        order.rotate_right(1);

        commands.entity(root_entity).replace_children(&order);
        commands.entity(root_entity).insert(MenuPranked);
    }
}
