use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), add_fade_out_music);
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[require(LevelComponent)]
#[component(on_add)]
pub struct MusicMarker {
    asset_path: String,
}

impl MusicMarker {
    pub fn new(asset_path: String) -> Self {
        Self { asset_path }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let marker = world.entity(ctx.entity).get::<Self>().unwrap();
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };
        let path = marker.asset_path.clone();
        let handle = asset_server.load(&path);

        world
            .commands()
            .entity(ctx.entity)
            .insert((Name::new(path), music(handle)));
    }
}

fn add_fade_out_music(mut commands: Commands, query: Query<Entity, With<MusicMarker>>) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(Fade::new(FadeMode::Out, Duration::from_secs_f32(3.0)));
    }
}
