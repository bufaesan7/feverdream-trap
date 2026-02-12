use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    window::PrimaryWindow,
};
use bevy_egui::{EguiContext, EguiContextSettings, EguiGlobalSettings, EguiPrimaryContextPass};
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, bevy_inspector::ui_for_assets};
use egui::LayerId;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use crate::{
    elements::{AssetHandleStash, EguiActionBuffer},
    prelude::*,
};

#[derive(Resource, Default)]
struct DescriptorPreview {
    descriptor: Option<Handle<ChunkDescriptor>>,
    level_entity: Option<Entity>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_egui::EguiPlugin::default());
    app.add_plugins(DefaultInspectorConfigPlugin);

    app.insert_resource(UiState::new());
    app.init_resource::<DescriptorPreview>();

    app.add_systems(OnEnter(Screen::Editor), setup);
    app.add_systems(
        EguiPrimaryContextPass,
        show_ui_system.run_if(in_state(Screen::Editor)),
    );
    app.add_systems(
        Update,
        update_descriptor_preview.run_if(in_state(Screen::Editor)),
    );
    app.add_systems(
        PostUpdate,
        set_camera_viewport
            .after(show_ui_system)
            .run_if(in_state(Screen::Editor)),
    );
}

fn setup(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
    egui_global_settings.auto_create_primary_context = false;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-15.0, 10.0, -15.0).looking_at(Vec3::new(0.0, 0., 0.0), Vec3::Y),
    ));

    // egui camera
    commands.spawn((
        Camera2d,
        Name::new("Egui Camera"),
        PrimaryEguiContext,
        RenderLayers::none(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

// make camera only render to view not obstructed by UI
fn set_camera_viewport(
    ui_state: Res<UiState>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut cam: Single<&mut Camera, Without<PrimaryEguiContext>>,
    egui_settings: Single<&EguiContextSettings>,
) {
    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let rect = physical_position + physical_size;

    let window_size = window.physical_size();
    if rect.x <= window_size.x && rect.y <= window_size.y {
        cam.viewport = Some(Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}

#[derive(Resource)]
struct UiState {
    state: DockState<EguiWindow>,
    viewport_rect: egui::Rect,
    pointer_in_viewport: bool,
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::GameView]);
        let tree = state.main_surface_mut();
        let [_game, sidebar_menu] =
            tree.split_left(NodeIndex::root(), 0.3, vec![EguiWindow::SidebarMenu]);
        let [_sidebar_menu, _options] =
            tree.split_below(sidebar_menu, 0.9, vec![EguiWindow::Options]);

        Self {
            state,
            viewport_rect: egui::Rect::NOTHING,
            pointer_in_viewport: false,
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            pointer_in_viewport: &mut self.pointer_in_viewport,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

#[derive(Debug)]
enum EguiWindow {
    GameView,
    SidebarMenu,
    Options,
}

struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    pointer_in_viewport: &'a mut bool,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        match window {
            EguiWindow::GameView => *self.viewport_rect = ui.clip_rect(),
            EguiWindow::SidebarMenu => {
                ui.vertical(|ui| {
                    // ------------------------------
                    // Chunk elements
                    // ------------------------------
                    ui.heading("ChunkElements");
                    ui_for_assets::<ChunkElement>(self.world, ui);
                    ui.separator();
                    let path = &mut self
                        .world
                        .resource_mut::<EguiActionBuffer>()
                        .new_element_name;
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(path);
                    });
                    if ui.button(format!("Create ChunkElement ({path})")).clicked() {
                        if path.is_empty() {
                            error!("Choose a more descriptive name!");
                        } else {
                            let path = path.clone();
                            let handle = self
                                .world
                                .resource::<AssetServer>()
                                .add(ChunkElement::new(path));
                            self.world
                                .resource_mut::<AssetHandleStash>()
                                .elements
                                .push(handle);
                        }
                    }
                    // ------------------------------
                    // Chunk descriptors
                    // ------------------------------
                    ui.separator();
                    ui.heading("ChunkDescriptors");
                    ui.separator();

                    // Preview buttons for each descriptor
                    ui.label("Preview descriptor:");
                    {
                        let descriptor_assets = self.world.resource::<Assets<ChunkDescriptor>>();
                        let asset_server = self.world.resource::<AssetServer>();
                        let current_preview = self
                            .world
                            .resource::<DescriptorPreview>()
                            .descriptor
                            .as_ref()
                            .map(|h| h.id());
                        let mut selected = None;
                        for (id, descriptor) in descriptor_assets.iter() {
                            let is_active = current_preview == Some(id);
                            let label = if is_active {
                                format!("[Active] {}", descriptor.name)
                            } else {
                                format!("Preview: {}", descriptor.name)
                            };
                            if ui.button(label).clicked() {
                                selected = Some(
                                    asset_server.get_id_handle::<ChunkDescriptor>(id).unwrap(),
                                );
                            }
                        }
                        if let Some(handle) = selected {
                            self.world.resource_mut::<DescriptorPreview>().descriptor =
                                Some(handle);
                        }
                    }
                    ui.separator();
                    let path = &mut self
                        .world
                        .resource_mut::<EguiActionBuffer>()
                        .new_descriptor_name;
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(path);
                    });
                    if ui
                        .button(format!("Create ChunkDescriptor ({path})"))
                        .clicked()
                    {
                        if path.is_empty() {
                            error!("Choose a more descriptive name!");
                        } else {
                            let path = path.clone();
                            let handle = self
                                .world
                                .resource::<AssetServer>()
                                .add(ChunkDescriptor::new(path));
                            self.world
                                .resource_mut::<AssetHandleStash>()
                                .descriptors
                                .push(handle);
                        }
                    }
                });
            }
            EguiWindow::Options => {
                ui.vertical(|ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Save Assets").clicked() {
                        info!("Saving assets");
                        // ------------------------------
                        // Chunk elements
                        // ------------------------------
                        let element_assets = self.world.resource::<Assets<ChunkElement>>();
                        for (_, element) in element_assets.iter() {
                            let element_asset = ChunkElementAsset::from(element);
                            let element_path = PathBuf::from("assets").join(element_asset.path());
                            let serialized_asset = to_string(&element_asset).unwrap();

                            info!("saving chunk element asset {}", element_path.display());
                            std::fs::write(element_path, serialized_asset).unwrap();
                        }
                        // ------------------------------
                        // Chunk descriptors
                        // ------------------------------
                        let descriptor_assets = self.world.resource::<Assets<ChunkDescriptor>>();
                        for (_, chunk) in descriptor_assets.iter() {
                            let chunk_asset = ChunkDescriptorAsset::from((chunk, element_assets));
                            let chunk_path = PathBuf::from("assets").join(chunk_asset.path());
                            let serialized_asset = to_string(&chunk_asset).unwrap();

                            info!("saving chunk asset {}", chunk_path.display());
                            std::fs::write(chunk_path, serialized_asset).unwrap();
                        }
                        info!("Saved assets");
                    }
                    if ui.button("Close editor without saving").clicked() {
                        self.world.write_message(AppExit::Success);
                    }
                });
            }
        }

        *self.pointer_in_viewport = ui
            .ctx()
            .rect_contains_pointer(LayerId::background(), self.viewport_rect.shrink(16.));
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}

fn update_descriptor_preview(
    mut commands: Commands,
    mut preview: ResMut<DescriptorPreview>,
    descriptor_assets: Res<Assets<ChunkDescriptor>>,
    element_assets: Res<Assets<ChunkElement>>,
) {
    if !preview.is_changed() {
        return;
    }

    // Despawn previous preview
    if let Some(level_entity) = preview.level_entity.take() {
        // Despawn the chunk via the DespawnChunk observer
        commands.trigger(DespawnChunk(ChunkId(0)));
        // Despawn the level entity itself
        commands.entity(level_entity).despawn();
    }

    // Spawn new preview if a descriptor is selected
    let Some(descriptor_handle) = preview.descriptor.as_ref() else {
        return;
    };

    let Some(descriptor) = descriptor_assets.get(descriptor_handle) else {
        return;
    };

    let elements: Vec<ChunkElement> = descriptor
        .elements
        .iter()
        .filter_map(|wrapper| element_assets.get(&wrapper.0).cloned())
        .collect();

    if elements.is_empty() {
        return;
    }

    let level_entity = commands
        .spawn((
            Name::new("Preview Level"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    commands.trigger(SpawnChunk {
        level: level_entity,
        id: ChunkId(0),
        grid_position: Vec2::ZERO,
        elements,
    });

    preview.level_entity = Some(level_entity);
}
