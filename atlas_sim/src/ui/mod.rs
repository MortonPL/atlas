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
    ui::{button_action, button_action_enabled, sidebar::SidebarPanel, window},
};
use internal::{
    export_world_state_clicked, import_world_clicked, import_world_state_clicked, load_config_clicked,
    reset_config_clicked, reset_panel_clicked, save_config_clicked, FileDialogHandler,
};
use panel::MainPanelGeneral;

use crate::{config::AtlasSimConfig, event::EventStruct};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<AtlasSimUi>()
            .add_systems(UiUpdate, update_ui)
            .add_systems(UiUpdate, update_viewport.after(update_ui));
    }
}

/// Update system
///
/// Redraw the immediate UI.
fn update_ui(
    mut config: ResMut<AtlasSimConfig>,
    mut contexts: EguiContexts,
    mut ui_base: ResMut<UiStateBase>,
    mut ui_state: ResMut<AtlasSimUi>,
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
struct AtlasSimUi {
    /// Is the simulations not running yet? Can we still make changes to the configuration?
    pub setup_mode: bool,
    /// Currently viewed sidebar panel.
    pub current_panel: Box<dyn SidebarPanel<AtlasSimConfig, EventStruct, Self> + Sync + Send>,
}

impl Default for AtlasSimUi {
    fn default() -> Self {
        Self {
            setup_mode: true,
            current_panel: Box::<MainPanelGeneral>::default(),
        }
    }
}

impl UiCreator<AtlasSimConfig, EventStruct> for AtlasSimUi {
    fn create_sidebar_head(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasSimConfig,
        ui_base: &mut UiStateBase,
        events: &mut EventStruct,
        exit: &mut EventWriter<AppExit>,
    ) {
        ui.vertical(|ui| {
            ui.heading(egui::RichText::new("Atlas History Simulator").size(24.0));
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    button_action_enabled(ui, "Import Generated World", self.setup_mode, || {
                        import_world_clicked(ui_base)
                    });
                    button_action_enabled(ui, "Import World State", self.setup_mode, || {
                        import_world_state_clicked(ui_base)
                    });
                    button_action(ui, "Export World State", || export_world_state_clicked(ui_base));
                    button_action(ui, "Exit", || exit.send(AppExit));
                });
                ui.menu_button("Edit", |ui| {
                    button_action(ui, "Reset Current Panel", || {
                        reset_panel_clicked(config, self, events)
                    });
                });
                ui.menu_button("Config", |ui| {
                    button_action(ui, "Save Configuration", || save_config_clicked(ui_base));
                    button_action_enabled(ui, "Load Configuration", self.setup_mode, || {
                        load_config_clicked(ui_base)
                    });
                    button_action_enabled(ui, "Reset Configuration", self.setup_mode, || {
                        reset_config_clicked(config, self)
                    });
                });
                ui.menu_button("Help", |ui| {
                    button_action(ui, "About", || ui_base.about_open = true);
                })
            });
        });
    }

    fn create_panel_tabs(&mut self, ui: &mut Ui, _ui_base: &mut UiStateBase, _events: &mut EventStruct) {
        ui.vertical(|ui| {
            let mut changed = false;
            egui::menu::bar(ui, |ui| {
                changed |= button_action(ui, "General", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
                changed |= button_action(ui, "Tab 2", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
                changed |= button_action(ui, "Tab 3", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
                changed |= button_action(ui, "Tab 4", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
            });
            egui::menu::bar(ui, |ui| {
                changed |= button_action(ui, "Tab 5", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
                changed |= button_action(ui, "Tab 6", || {
                    self.current_panel = Box::<MainPanelGeneral>::default();
                    true
                });
            });
            if changed {
                // TODO
            }
        });
    }

    fn create_current_panel(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig, events: &mut EventStruct) {
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
        config: &mut AtlasSimConfig,
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
