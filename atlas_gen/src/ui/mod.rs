mod internal;
mod panels;

use atlas_lib::{
    base::{
        events::EventStruct,
        ui::{
            open_file_dialog, update_viewport, FileDialogMode, HandleFileDialog, UiCreator, UiPluginBase,
            UiStateBase, UiUpdate,
        },
    },
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*},
    bevy_egui::{
        egui::{self, Context, RichText, Ui},
        EguiContexts,
    },
    config::gen::AtlasGenConfig,
    ui::{button_action, sidebar::SidebarPanel, window},
};
use internal::{clear_layer_clicked, reset_config_clicked, reset_panel_clicked, FileDialogHandler};
use panels::*;

/// Plugin responsible for the entire GUI and viewport rectangle.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<AtlasGenUi>()
            .add_systems(UiUpdate, (update_ui, update_viewport).chain());
    }
}

/// Update system
///
/// Redraw the immediate UI.
fn update_ui(
    mut config: ResMut<AtlasGenConfig>,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<AtlasGenUi>,
    mut ui_base: ResMut<UiStateBase>,
    mut events: ResMut<EventStruct>,
    mut exit: EventWriter<AppExit>,
    window: Query<&Window>,
) {
    if !window.single().focused {
        return;
    }
    ui_state.create_ui(
        contexts.ctx_mut(),
        &mut config,
        &mut ui_base,
        &mut events,
        &mut exit,
    );
}

#[derive(Resource)]
struct AtlasGenUi {
    /// Currently viewed sidebar panel.
    pub current_panel: Box<dyn SidebarPanel<AtlasGenConfig, Self> + Sync + Send>,
}

impl Default for AtlasGenUi {
    fn default() -> Self {
        Self {
            current_panel: Box::<MainPanelGeneral>::default(),
        }
    }
}

impl UiCreator<AtlasGenConfig> for AtlasGenUi {
    fn create_sidebar_head(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        ui_base: &mut UiStateBase,
        events: &mut EventStruct,
        exit: &mut EventWriter<AppExit>,
    ) {
        ui.vertical(|ui| {
            ui.heading(egui::RichText::new("Atlas Map Generator").size(24.0));
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    button_action(ui, "Import World", || {
                        open_file_dialog(ui_base, FileDialogMode::Import)
                    });
                    button_action(ui, "Export World", || {
                        open_file_dialog(ui_base, FileDialogMode::Export)
                    });
                    button_action(ui, "Exit", || {
                        exit.send(AppExit);
                    });
                });
                ui.menu_button("Edit", |ui| {
                    button_action(ui, "Reset Current Panel", || {
                        reset_panel_clicked(config, self, events)
                    });
                });
                ui.menu_button("Config", |ui| {
                    button_action(ui, "Save Configuration", || {
                        open_file_dialog(ui_base, FileDialogMode::SaveConfig)
                    });
                    button_action(ui, "Load Configuration", || {
                        open_file_dialog(ui_base, FileDialogMode::LoadConfig)
                    });
                    button_action(ui, "Reset Configuration", || {
                        reset_config_clicked(config, self, events)
                    });
                });
                ui.menu_button("Layer", |ui| {
                    button_action(ui, "Load Layer Data", || {
                        open_file_dialog(ui_base, FileDialogMode::LoadData(ui_base.current_layer))
                    });
                    button_action(ui, "Save Layer Data", || {
                        open_file_dialog(ui_base, FileDialogMode::SaveData(ui_base.current_layer))
                    });
                    button_action(ui, "Clear Layer Data", || clear_layer_clicked(ui_base, events));
                    button_action(ui, "Render Layer Image", || {
                        open_file_dialog(ui_base, FileDialogMode::RenderImage(ui_base.current_layer))
                    });
                });
                ui.menu_button("Help", |ui| {
                    button_action(ui, "About", || ui_base.about_open = true);
                })
            });
        });
    }

    fn create_panel_tabs(&mut self, ui: &mut Ui, ui_base: &mut UiStateBase, events: &mut EventStruct) {
        ui.vertical(|ui| {
            let mut changed = false;
            // Tab shorthand macro.
            macro_rules! tab {
                ($text:literal, $class:ty, $ui:expr) => {
                    changed |= button_action($ui, $text, || {
                        self.current_panel = Box::<$class>::default();
                        true
                    });
                };
            }
            egui::menu::bar(ui, |ui| {
                tab!("General", MainPanelGeneral, ui);
                tab!("Continents", MainPanelContinents, ui);
                tab!("Topography", MainPanelTopography, ui);
                tab!("Temperature", MainPanelTemperature, ui);
            });
            egui::menu::bar(ui, |ui| {
                tab!("Precipitation", MainPanelPrecipitation, ui);
                tab!("Climate", MainPanelClimate, ui);
                tab!("Deposits", MainPanelDeposits, ui);
            });
            if changed {
                let layer = self.current_panel.get_layer();
                events.viewed_layer_changed = Some(layer);
                ui_base.current_layer = layer;
            }
        });
    }

    fn create_current_panel(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig, events: &mut EventStruct) {
        // Panel heading.
        ui.heading(self.current_panel.get_heading());
        // Panel inner.
        egui::ScrollArea::both().show(ui, |ui| {
            self.current_panel.clone_box().show(ui, config, self, events);
            ui.separator(); // HACK! Again! Without it the scroll area isn't greedy.
        });
    }

    fn handle_about(ctx: &Context, name: impl Into<RichText>, open: &mut bool) {
        window(ctx, "About", open, |ui| {
            ui.heading(name);
            ui.label(env!("CARGO_PKG_DESCRIPTION"));
            ui.separator();
            ui.label(format!("Authors: {}", env!("CARGO_PKG_AUTHORS")));
            ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
            ui.label(format!("Home Page: {}", env!("CARGO_PKG_HOMEPAGE")));
        });
    }

    fn handle_file_dialog(
        config: &mut AtlasGenConfig,
        events: &mut EventStruct,
        ctx: &Context,
        ui_base: &mut UiStateBase,
    ) {
        FileDialogHandler::new(events, config).handle(ctx, ui_base);
        if let Some(error) = events.error_window.take() {
            ui_base.error_message = error;
            ui_base.error_window_open = true;
        }
    }
}
