use bevy_egui::egui::Ui;

use atlas_lib::ui::{button, sidebar::MakeUi};

use crate::{
    config::{InfluenceShape, SessionConfig},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{
            simple::{MainPanelClimate, MainPanelContinents},
            MainPanel, MainPanelTransition,
        },
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        config.topography.make_ui(ui);

        if !matches!(config.topography.influence_map_type, InfluenceShape::None(_))
            && button(ui, "Generate Influence Map")
        {
            events.generate_request = Some(ViewedMapLayer::TopographyInfluence);
        }
        if button(ui, "Generate Layer") {
            events.generate_request = Some(self.get_layer());
        }
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Topography
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelContinents>::default(),
            MainPanelTransition::Next => Box::<MainPanelClimate>::default(),
        }
    }
}
