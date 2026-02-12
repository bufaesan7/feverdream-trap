use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EguiActionBuffer>();
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EguiActionBuffer {
    pub new_element_name: String,
    pub new_descriptor_name: String,
}
