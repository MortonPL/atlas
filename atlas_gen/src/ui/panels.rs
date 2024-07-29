use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    config::r#gen::{AtlasGenConfig, InfluenceShape},
    domain::map::MapDataLayer,
    ui::{
        button,
        sidebar::{MakeUi, SidebarPanel},
        UiEditableEnum,
    },
};

use super::{internal::SidebarPanelGen, AtlasGenUi};

macro_rules! make_panel {
    ($cls:ident, $enm:ident, $field:ident) => {
        #[derive(Default, Clone, Copy)]
        pub struct $cls;

        impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for $cls {
            fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
                config.$field.make_ui(ui);
            }

            fn extra_ui_pre(
                &mut self,
                ui: &mut Ui,
                config: &mut AtlasGenConfig,
                _ui_state: &mut AtlasGenUi,
                events: &mut EventStruct,
            ) {
                self.button_influence(ui, events, self.get_influence_shape(config));
                self.button_layer(ui, events);
            }

            fn get_heading(&self) -> &'static str {
                stringify!($enm)
            }

            fn get_layer(&self) -> MapDataLayer {
                MapDataLayer::$enm
            }
        }

        impl SidebarPanelGen for $cls {
            fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
                &config.$field.influence_shape
            }
        }
    };
}

// Panel with continents generation settings.
make_panel!(MainPanelContinents, Continents, continents);

// Panel with topography generation settings.
make_panel!(MainPanelTopography, Topography, topography);

// Panel with topography generation settings.
make_panel!(MainPanelTemperature, Temperature, temperature);

// Panel with precipitation generation settings.
make_panel!(MainPanelPrecipitation, Precipitation, precipitation);

/// Panel with general world gen and preview settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {}

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelGeneral {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        let old_world_model = config.general.preview_model.self_as_index();
        let old = config.general.world_size;

        if button(ui, "Generate Preview") {
            events.generate_request = Some((self.get_layer(), false));
        }

        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            config.general.make_ui(ui);
        });

        if config.general.preview_model.self_as_index() != old_world_model {
            events.world_model_changed = Some(());
        }

        if old != config.general.world_size {
            events.world_model_changed = Some(());
        }
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

impl SidebarPanelGen for MainPanelGeneral {}

/// Panel with climate generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        if ui.button("Reload \"climatemap.png\"").clicked() {
            events.load_climatemap_request = Some(());
        }

        self.button_layer(ui, events);

        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            self.make_ui(ui, config);
        });
    }

    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.climate.make_ui(ui);
        if config.climate.mountains_biome as usize >= config.climate.biomes.len() {
            config.climate.mountains_biome = 0
        }
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }
}

impl SidebarPanelGen for MainPanelClimate {}

/// Panel with resource deposit generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelDeposits;

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelDeposits {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.deposits.make_ui(ui);
    }

    fn extra_ui_pre(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

impl SidebarPanelGen for MainPanelDeposits {
    /// Create a "Generate Layer" button.
    fn button_layer(&self, ui: &mut Ui, events: &mut EventStruct) {
        if button(ui, "Generate Layer") {
            events.generate_request = Some((MapDataLayer::Deposits, false));
        }
    }
}
