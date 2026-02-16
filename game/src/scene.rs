use bevy::scene::SceneInstanceReady;
#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;

use crate::camera_controller::{
    CameraMarker, CameraStatusEffects, CameraTargetCharacterController, spawn_camera,
};
use crate::character_controller::{Player, PlayerInput, spawn_player};
use crate::interaction::Interactable;
use crate::prelude::*;
use crate::utils::audio::MusicMarker;
use bevy::ecs::system::RunSystemOnce;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CurrentLevel>()
        .load_resource::<GameSceneStorage>()
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
    /// When true, skip saving scene on exit (e.g., when transitioning between levels).
    pub skip_save: bool,
}

impl FromWorld for GameSceneStorage {
    fn from_world(world: &mut World) -> Self {
        if std::fs::exists(scene_file_path()).is_ok_and(|b| b) {
            let asset_server = world.resource::<AssetServer>();
            Self {
                handle: Some(asset_server.load(SCENE_FILE)),
                skip_save: false,
            }
        } else {
            Self {
                handle: None,
                skip_save: false,
            }
        }
    }
}

// see https://github.com/bevyengine/bevy/blob/e31f01174714b68738692c259837e59f37797096/examples/scene/scene.rs#L158
fn save_scene(
    world: &World,
    mut commands: Commands,
    query: Query<Entity, With<LevelComponent>>,
    game_scene: Res<GameSceneStorage>,
) {
    // Skip saving when transitioning between levels
    if game_scene.skip_save {
        return;
    }
    // This is a closure because neither `DynamicSceneBuilder` nor `DynamicScene` implement `Clone`
    let scene = || {
        DynamicSceneBuilder::from_world(world)
            //
            // Allowed resources
            //
            .allow_resource::<CameraStatusEffects>()
            //
            // Allowed components
            //
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
            // Chunk Components
            .allow_component::<SpawnMarker>()
            .allow_component::<ChunkLight>()
            .allow_component::<SwapSensorChunk>()
            .allow_component::<ReplaceAssetSensorChunk>()
            .allow_component::<MoveChunkSensorChunk>()
            // Relationships
            .allow_component::<Children>()
            .allow_component::<ChildOf>()
            // Interactions
            .allow_component::<Interactable>()
            .allow_component::<DespawnInteraction>()
            .allow_component::<SwapChunksInteraction>()
            .allow_component::<PlaySoundEffectInteraction>()
            // Audio
            .allow_component::<MusicMarker>()
            //
            // Extraction
            //
            .extract_entities(query.iter())
            .extract_resources()
            .build()
    };

    let handle = world.resource::<AssetServer>().add(scene());
    commands.insert_resource(GameSceneStorage {
        handle: Some(handle),
        skip_save: false,
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

fn spawn_scene(mut commands: Commands, mut game_scene: ResMut<GameSceneStorage>) {
    // Reset skip_save flag for this gameplay session
    game_scene.skip_save = false;

    commands.init_resource::<CameraStatusEffects>();

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
            let _ = world.run_system_once(spawn_music);
            let _ = world.run_system_once(spawn_text);
        });
    }
}

fn on_level_spawned(event: On<Add, Level>, mut commands: Commands) {
    let entity = event.event_target();

    commands
        .entity(entity)
        .insert(DespawnOnExit(Screen::Gameplay));
}

fn on_level_component_spawned(
    event: On<Add, LevelComponent>,
    mut commands: Commands,
    music_marker: Query<(), With<MusicMarker>>,
) {
    let entity = event.event_target();

    // Kinda hacky, but we are ignoring Music Marker here, so we can handle a fade out
    // OnExit(Screen::Gameplay) somewhere else
    if music_marker.contains(entity) {
        return;
    }

    commands
        .entity(entity)
        .insert(DespawnOnExit(Screen::Gameplay));
}

fn spawn_music(mut commands: Commands) {
    commands.spawn(MusicMarker::new(String::from(
        "audio/music/Heavenly Loop.ogg",
    )));
}

fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Text"),
        Text::new("Oh my god, dont let me go."),
        Fade::new(FadeMode::Out, Duration::from_secs_f32(4.0)),
    ));
}
