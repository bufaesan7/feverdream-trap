#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<GameSceneStorage>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_scene)
        .add_systems(OnExit(Screen::Gameplay), save_scene);
}

pub const SCENE_FILE: &str = "game_state.scn.ron";
/// TODO! Change for windows
pub const SCENE_FILE_PATH: &str = "assets/game_state.scn.ron";

#[derive(Asset, TypePath, Resource, Clone)]
pub struct GameSceneStorage {
    #[dependency]
    pub scene: Handle<DynamicScene>,
}

impl FromWorld for GameSceneStorage {
    fn from_world(world: &mut World) -> Self {
        if std::fs::exists(SCENE_FILE_PATH).is_ok_and(|b| b) {
            let asset_server = world.resource::<AssetServer>();
            Self {
                scene: asset_server.load(SCENE_FILE),
            }
        } else {
            let mut scene_world = World::new();
            let type_registry = world.resource::<AppTypeRegistry>().clone();
            scene_world.insert_resource(type_registry);
            // TODO! Spawn default scene

            let scene = DynamicScene::from_world(&scene_world);
            let mut scenes = world.resource_mut::<Assets<DynamicScene>>();
            Self {
                scene: scenes.add(scene),
            }
        }
    }
}

// see https://github.com/bevyengine/bevy/blob/e31f01174714b68738692c259837e59f37797096/examples/scene/scene.rs#L158
fn save_scene(world: &mut World) {
    let mut scene_world = World::new();

    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    let scene = DynamicScene::from_world(&scene_world);
    let handle = world.resource::<AssetServer>().add(scene);

    let mut scene_storage = world.resource_mut::<GameSceneStorage>();
    scene_storage.scene = handle;

    // DynamicScene does not implement clone
    let scene = DynamicScene::from_world(&scene_world);
    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    // This can't work in WASM as there is no filesystem access.
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file

            use std::{fs::File, io::Write as _};
            File::create(SCENE_FILE_PATH)
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

fn spawn_scene(mut commands: Commands, game_scene: Res<GameSceneStorage>) {
    commands.spawn((
        Name::new("Game Scene"),
        DynamicSceneRoot(game_scene.scene.clone()),
        DespawnOnExit(Screen::Gameplay),
    ));
}
