use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};

use crate::{
    config::{FlatWorldModel, GeneratorConfig, GlobeWorldModel, WorldModel},
    map::ViewedMapLayer,
};

use atlas_lib::ui::{UiControl, UiSlider, UiSliderN};

use super::{
    internal::{MainPanel, UiState},
    panel_continents::MainPanelContinents,
    utils::add_section,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl MainPanel for MainPanelGeneral {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        let old_model_globe = match &config.general.world_model {
            WorldModel::Flat(_) => false,
            WorldModel::Globe(_) => true,
        };
        let mut model_globe = old_model_globe;
        add_section(ui, "World", |ui| {
            ui.label("Seed").on_hover_text_at_pointer("TODO");
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut config.general.seed)
                        .speed(10)
                        .clamp_range(u32::MIN..=u32::MAX),
                )
                .on_hover_text_at_pointer("TODO");
                if ui.button("Random").clicked() {
                    config.general.seed = rand::random()
                }
            });
            ui.end_row();

            ui.label("World Model").on_hover_text_at_pointer("TODO");
            egui::ComboBox::from_label("")
                .selected_text(config.general.world_model.str())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut model_globe, false, "Flat");
                    ui.selectable_value(&mut model_globe, true, "Globe")
                })
                .response
                .on_hover_text_at_pointer("TODO");
            ui.end_row();

            UiSlider::new(ui, "Tile resolution", &mut config.general.tile_resolution)
                .clamp_range(10.0..=200.0)
                .show(None);
        });
        if old_model_globe != model_globe {
            config.general.world_model = match model_globe {
                false => WorldModel::Flat(Default::default()),
                true => WorldModel::Globe(Default::default()),
            };
        }
        add_section(
            ui,
            format!("{} World Settings", config.general.world_model.str()),
            |ui| {
                match &mut config.general.world_model {
                    WorldModel::Flat(x) => create_general_flat_settings(ui, ui_state, x),
                    WorldModel::Globe(x) => create_general_globe_settings(ui, ui_state, x),
                };
            },
        );
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn transition(&self, _prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if next {
            Box::<MainPanelContinents>::default()
        } else {
            Box::new(*self)
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::None
    }
}

fn create_general_flat_settings(ui: &mut Ui, ui_state: &mut UiState, config: &mut FlatWorldModel) {
    let old = config.world_size;
    UiSliderN::new(ui, "World Size", &mut config.world_size)
        .clamp_range(100..=500)
        .show(None);
    ui_state.just_changed_dimensions = old != config.world_size;
}

fn create_general_globe_settings(
    _ui: &mut Ui,
    _ui_state: &mut UiState,
    _config: &mut GlobeWorldModel,
) {
    // TODO
}
