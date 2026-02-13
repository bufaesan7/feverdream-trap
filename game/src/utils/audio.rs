use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
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
