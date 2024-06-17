mod internal;
mod panel;

use atlas_lib::{
    base::ui::{update_viewport, HandleFileDialog, UiCreator, UiPluginBase, UiStateBase, UiUpdate},
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*},
    bevy_egui::{
        egui::{self, Context, RichText, Ui},
        EguiContexts,
    },
    domain::map::MapDataLayer,
    ui::{button_action, sidebar::SidebarPanel, window},
};
use internal::{
    clear_layer_clicked, export_world_clicked, import_world_clicked, load_config_clicked, load_layer_clicked,
    render_layer_clicked, reset_config_clicked, reset_panel_clicked, save_config_clicked, save_layer_clicked,
    FileDialogHandler,
};
use panel::{
    MainPanelClimate, MainPanelContinents, MainPanelGeneral, MainPanelPrecipitation, MainPanelTemperature,
    MainPanelTopography,
};

use crate::{config::AtlasGenConfig, event::EventStruct};

/// Plugin responsible for the entire GUI and viewport rectangle.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<AtlasGenUi>()
            .add_systems(UiUpdate, update_ui)
            .add_systems(UiUpdate, update_viewport.after(update_ui));
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
    pub current_panel: Box<dyn SidebarPanel<AtlasGenConfig, EventStruct, Self> + Sync + Send>,
}

impl Default for AtlasGenUi {
    fn default() -> Self {
        Self {
            current_panel: Box::<MainPanelGeneral>::default(),
        }
    }
}

impl UiCreator<AtlasGenConfig, EventStruct> for AtlasGenUi {
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
                    button_action(ui, "Import World", || import_world_clicked(ui_base));
                    button_action(ui, "Export World", || export_world_clicked(ui_base));
                    button_action(ui, "Exit", || exit.send(AppExit));
                });
                ui.menu_button("Edit", |ui| {
                    button_action(ui, "Reset Current Panel", || {
                        reset_panel_clicked(config, self, events)
                    });
                });
                ui.menu_button("Config", |ui| {
                    button_action(ui, "Save Configuration", || save_config_clicked(ui_base));
                    button_action(ui, "Load Configuration", || load_config_clicked(ui_base));
                    button_action(ui, "Reset Configuration", || {
                        reset_config_clicked(config, self, events)
                    });
                });
                ui.menu_button("Layer", |ui| {
                    button_action(ui, "Load Layer Data", || load_layer_clicked(ui_base));
                    button_action(ui, "Save Layer Data", || save_layer_clicked(ui_base));
                    button_action(ui, "Clear Layer Data", || clear_layer_clicked(ui_base, events));
                    button_action(ui, "Render Layer Image", || render_layer_clicked(ui_base));
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
            egui::menu::bar(ui, |ui| {
                changed |= button_action(ui, "General", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
                changed |= button_action(ui, "Continents", || {
                    self.current_panel = Box::<MainPanelContinents>::default();
                    true
                });
                changed |= button_action(ui, "Topography", || {
                    self.current_panel = Box::<MainPanelTopography>::default();
                    true
                });
                changed |= button_action(ui, "Temperature", || {
                    self.current_panel = Box::<MainPanelTemperature>::default();
                    true
                });
            });
            egui::menu::bar(ui, |ui| {
                changed |= button_action(ui, "Precipitation", || {
                    self.current_panel = Box::<MainPanelPrecipitation>::default();
                    true
                });
                changed |= button_action(ui, "Climate", || {
                    self.current_panel = Box::<MainPanelClimate>::default();
                    true
                });
                /* TODO
                changed |= button_action(ui, "Resources", || {
                    ui_panel.current_panel = Box::<MainPanelResources>::default();
                    true
                });
                */
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

    fn notify_viewed_layer_changed(events: &mut EventStruct, layer: MapDataLayer) {
        events.viewed_layer_changed = Some(layer);
    }
}
