use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    domain::graphics::color_to_u8,
    ui::sidebar::SidebarControl,
    MakeUi,
};

/// Civilization simulation.
pub struct CivPlugin;

impl Plugin for CivPlugin {
    fn build(&self, app: &mut App) {
        // TODO
    }
}

#[derive(Component)]
pub struct Civ {
    /// Map color.
    pub color: Color,
}

#[derive(Component, MakeUi)]
pub struct CivUi {
    #[name("Color")]
    #[control(SidebarColor)]
    /// Map color.
    pub color: [u8; 3],
}

impl From<&Civ> for CivUi {
    fn from(value: &Civ) -> Self {
        Self {
            color: color_to_u8(&value.color),
        }
    }
}

impl Default for Civ {
    fn default() -> Self {
        Self {
            color: Default::default(),
        }
    }
}
