use bevy::scene::SceneInstanceReady;
#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;

use crate::camera_controller::{CameraMarker, CameraTargetCharacterController, spawn_camera};
use crate::character_controller::{Player, PlayerInput, spawn_player};
use crate::interaction::Interactable;
use crate::prelude::*;
use bevy::ecs::system::RunSystemOnce;

pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<GameSceneStorage>()
        .add_observer(on_level_spawned)
        .add_observer(on_level_component_spawned)
        .add_systems(OnEnter(Screen::Gameplay), spawn_scene)
        .add_systems(OnExit(Screen::Gameplay), save_scene);
}

pub const SCENE_FILE: &str = "game_state.scn.ron";

pub fn scene_file_path() -> std::path::PathBuf {
    std::path::Path::new("assets").join(SCENE_FILE)
}

#[derive(Asset, TypePath, Resource, Clone)]
/// Either `handle` or `scene` are guaranteed to exist
/// `scene` is necessary, because the [`Asset`] won't load if we assign a [`Handle`] to `handle`
/// that isn't loaded by the [`AssetServer`].
pub struct GameSceneStorage {
    #[dependency]
    pub handle: Option<Handle<DynamicScene>>,
}

impl FromWorld for GameSceneStorage {
    fn from_world(world: &mut World) -> Self {
        if std::fs::exists(scene_file_path()).is_ok_and(|b| b) {
            let asset_server = world.resource::<AssetServer>();
            Self {
                handle: Some(asset_server.load(SCENE_FILE)),
            }
        } else {
            Self { handle: None }
        }
    }
}

// see https://github.com/bevyengine/bevy/blob/e31f01174714b68738692c259837e59f37797096/examples/scene/scene.rs#L158
fn save_scene(world: &World, mut commands: Commands, query: Query<Entity, With<LevelComponent>>) {
    // This is a closure because neither `DynamicSceneBuilder` nor `DynamicScene` implement `Clone`
    let scene = || {
        DynamicSceneBuilder::from_world(world)
            .allow_component::<Name>()
            .allow_component::<Level>()
            .allow_component::<LevelComponent>()
            .allow_component::<LevelCollider>()
            .allow_component::<LevelComponent3d>()
            .allow_component::<LevelComponentGltf>()
            .allow_component::<Transform>()
            .allow_component::<Visibility>()
            .allow_component::<CameraMarker>()
            .allow_component::<CameraTargetCharacterController>()
            .allow_component::<Player>()
            .allow_component::<PlayerInput>()
            // Physics
            .allow_component::<Collider>()
            .allow_component::<CollisionEventsEnabled>()
            .allow_component::<CollisionLayers>()
            .allow_component::<Sensor>()
            .allow_component::<RigidBody>()
            // Chunks
            .allow_component::<Chunk>()
            .allow_component::<ChunkId>()
            .allow_component::<SwapSensorChunk>()
            .allow_component::<ReplaceAssetSensorChunk>()
            // Relationships
            .allow_component::<Children>()
            .allow_component::<ChildOf>()
            .extract_entities(query.iter())
            // Interactions
            .allow_component::<Interactable>()
            .allow_component::<DespawnInteraction>()
            .allow_component::<SwapChunksInteraction>()
            .build()
    };

    let handle = world.resource::<AssetServer>().add(scene());
    commands.insert_resource(GameSceneStorage {
        handle: Some(handle),
    });

    // This can't work in WASM as there is no filesystem access.
    #[cfg(not(target_arch = "wasm32"))]
    {
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        let serialized_scene = scene().serialize(&type_registry).unwrap();

        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file

                use std::{fs::File, io::Write as _};
                File::create(scene_file_path())
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}

fn spawn_scene(mut commands: Commands, game_scene: Res<GameSceneStorage>) {
    if let Some(handle) = &game_scene.handle {
        // Load saved scene
        commands
            .spawn((
                Name::new("Game scene spawner"),
                Transform::default(),
                Visibility::default(),
                DynamicSceneRoot(handle.clone()),
                DespawnOnExit(Screen::Gameplay),
            ))
            .observe(|event: On<SceneInstanceReady>, mut commands: Commands| {
                commands.entity(event.entity).detach_all_children();
            });
    } else {
        // No saved scene, spawn from layout
        commands.queue(|world: &mut World| {
            let _ = world.run_system_once(spawn_level_from_layout);
            let _ = world.run_system_once(spawn_camera);
            let _ = world.run_system_once(spawn_player);
        });
    }
}

fn on_level_spawned(event: On<Add, Level>, mut commands: Commands) {
    let entity = event.event_target();

    commands
        .entity(entity)
        .insert(DespawnOnExit(Screen::Gameplay));
}

fn on_level_component_spawned(event: On<Add, LevelComponent>, mut commands: Commands) {
    let entity = event.event_target();

    commands
        .entity(entity)
        .insert(DespawnOnExit(Screen::Gameplay));
}
