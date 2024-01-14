use bevy::prelude::*;
use bevy_egui::egui::{self, Ui, RichText};

use crate::{config::GeneratorConfig, map::ViewedMapLayer};

use super::{
    internal::{MainPanel, FileDialogMode, UiState, ImageLayer}, panel_topography::MainPanelTopography, panel_general::MainPanelGeneral, utils::add_section,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl MainPanel for MainPanelContinents {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        if ui.add(egui::Button::new(RichText::new("Load Continents").size(24.0))).clicked() {
            let mut file_picker = egui_file::FileDialog::open_file(None);
            file_picker.open();
            ui_state.file_dialog = Some(file_picker);
            ui_state.file_dialog_mode = FileDialogMode::LoadImage(ImageLayer::Continental);
        }
        add_section(ui, "Continents Generator", |ui| {

        });
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn transition(&self, prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if prev {
            Box::new(MainPanelGeneral::default())
        } else if next {
            Box::new(MainPanelTopography::default())
        } else {
            Box::new(*self)
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Continental
    }
}
