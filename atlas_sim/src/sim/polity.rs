use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
    },
    config::AtlasConfig,
};

use crate::config::AtlasSimConfig;

/// Plugin responsible for the actual simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals);
    }
}

/// Ownership status of a polity.
#[derive(Default)]
pub enum Ownership {
    /// This polity is independent and has no master.
    #[default]
    Independent,
    /// This polity has a master but keeps local autonomy.
    Autonomous(Entity),
    /// This polity has a master and no local ruler.
    Integrated(Entity),
    /// This polity is occupied by an external force.
    Occupied(Entity),
}

/// A political entity that owns land and population.
#[derive(Component)]
pub struct Polity {
    /// Map tile indices that this polity owns.
    pub tiles: Vec<u32>,
    /// Centroid of owned land, in map coords.
    pub centroid: Vec2,
    /// XYWH bounding box in map coordinates.
    pub xywh: [u32; 4],
    /// Ownership status.
    pub ownership: Ownership,
    /// Polity map color.
    pub color: [u8; 3],
    /// Visuals need to be updated due to color or shape changes.
    pub need_visual_update: bool,
}

/// Update system
///
/// Update polity visuals.
fn update_visuals(
    config: Res<AtlasSimConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut Polity, &mut Transform, &mut Handle<StandardMaterial>), With<Polity>>,
) {
    for (mut polity, mut tran, mut mat) in query.iter_mut() {
        // Don't update if not needed.
        if !polity.need_visual_update {
            continue;
        }
        let (w, _) = config.get_world_size();
        let (x, y, width, height) = (polity.xywh[0], polity.xywh[1], polity.xywh[2], polity.xywh[3]);
        // Make new texture data.
        let (off, diff) = (w * y + x, w - width);
        let mut data = vec![0; width as usize * height as usize * 4];
        for i in &polity.tiles {
            let i = i - off;
            let i = ((i - diff * (i / w)) * 4) as usize;
            data[i] = 255;
            data[i + 1] = 255;
            data[i + 2] = 255;
            data[i + 3] = 255;
        }
        // Get world space origin and scale.
        let p = config.centroid_to_world_centered(polity.centroid.into());
        let s = (width as f32 / 100.0, height as f32 / 100.0);
        tran.translation = Vec3::new(p.0, p.1, 0.0);
        tran.scale = Vec3::new(s.0, s.1, s.1);
        // Update the material (with tint) and texture (with shape).
        *mat = materials.add(StandardMaterial {
            base_color: Color::rgb_u8(polity.color[0], polity.color[1], polity.color[2]),
            base_color_texture: Some(images.add(Image::new(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                data,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::all(),
            ))),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        polity.need_visual_update = false;
    }
}
