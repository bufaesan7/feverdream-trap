use bevy::scene::SceneInstanceReady;
#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;

use crate::chunk::{Chunk, ChunkId, SwapSensorChunk, SwappableChunk};
use crate::level::LevelCollider;
use crate::{
    camera_controller::{CameraMarker, CameraTargetCharacterController, spawn_camera},
    character_controller::{Player, PlayerInput, spawn_player},
    interaction::Interactable,
    level::{Level, LevelComponent, LevelComponent3d, spawn_level},
    prelude::*,
};

pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<GameSceneStorage>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_scene)
        .add_systems(OnExit(Screen::Gameplay), save_scene);
}

pub const SCENE_FILE: &str = "game_state.scn.ron";
#[cfg(not(target_os = "windows"))]
pub const SCENE_FILE_PATH: &str = "assets/game_state.scn.ron";
#[cfg(target_os = "windows")]
pub const SCENE_FILE_PATH: &str = "assets\\game_state.scn.ron";

#[derive(Asset, TypePath, Resource, Clone)]
/// Either `handle` or `scene` are guaranteed to exist
/// `scene` is necessary, because the [`Asset`] won't load if we assign a [`Handle`] to `handle`
/// that isn't loaded by the [`AssetServer`].
pub struct GameSceneStorage {
    #[dependency]
    pub handle: Option<Handle<DynamicScene>>,
    pub scene: Option<Handle<DynamicScene>>,
}

impl FromWorld for GameSceneStorage {
    fn from_world(world: &mut World) -> Self {
        if std::fs::exists(SCENE_FILE_PATH).is_ok_and(|b| b) {
            let asset_server = world.resource::<AssetServer>();
            Self {
                handle: Some(asset_server.load(SCENE_FILE)),
                scene: None,
            }
        } else {
            let mut scene_world = World::new();
            let type_registry = world.resource::<AppTypeRegistry>().clone();
            scene_world.insert_resource(type_registry);

            fn run_system<M>(
                scene_world: &mut World,
                system: impl IntoSystem<(), (), M> + 'static,
            ) {
                let system_id = scene_world.register_system(system);
                scene_world.run_system(system_id).unwrap();
            }

            run_system(&mut scene_world, spawn_level);
            run_system(&mut scene_world, spawn_camera);
            run_system(&mut scene_world, spawn_player);

            let scene = DynamicScene::from_world(&scene_world);
            let mut scenes = world.resource_mut::<Assets<DynamicScene>>();
            Self {
                handle: None,
                scene: Some(scenes.add(scene)),
            }
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
            .allow_component::<SwappableChunk>()
            .allow_component::<SwapSensorChunk>()
            // Relationships
            .allow_component::<Children>()
            .allow_component::<ChildOf>()
            .allow_component::<Interactable>()
            .extract_entities(query.iter())
            .build()
    };

    let handle = world.resource::<AssetServer>().add(scene());
    commands.insert_resource(GameSceneStorage {
        handle: Some(handle),
        scene: None,
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
                File::create(SCENE_FILE_PATH)
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}

fn spawn_scene(mut commands: Commands, game_scene: Res<GameSceneStorage>) {
    let handle = if let Some(handle) = &game_scene.handle {
        handle.clone()
    } else {
        game_scene.scene.clone().unwrap()
    };
    // This will sometimes trigger a `B0004` warning, that's a bevy bug:
    // https://github.com/bevyengine/bevy/pull/22675
    commands
        .spawn((
            Name::new("Game scene spawner"),
            Transform::default(),
            Visibility::default(),
            DynamicSceneRoot(handle),
            DespawnOnExit(Screen::Gameplay),
        ))
        .observe(|event: On<SceneInstanceReady>, mut commands: Commands| {
            commands.entity(event.entity).detach_all_children();
        });
}
