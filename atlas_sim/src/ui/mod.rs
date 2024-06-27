mod internal;
mod panel;

use atlas_lib::{
    base::{
        events::EventStruct,
        ui::{
            open_file_dialog, update_viewport, FileDialogMode, HandleFileDialog, MainCamera, UiCreator,
            UiPluginBase, UiStateBase, UiUpdate,
        },
    },
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*, window::PrimaryWindow},
    bevy_egui::{
        egui::{self, Context, RichText, Ui},
        EguiContexts,
    },
    config::AtlasConfig,
    domain::graphics::CurrentWorldModel,
    ui::{
        button_action, button_action_enabled,
        sidebar::{SidebarControl, SidebarEnumDropdown, SidebarPanel},
        window,
    },
};
use internal::{reset_config_clicked, reset_panel_clicked, FileDialogHandler};
use panel::{MainPanelGeneral, MainPanelScenario};

use crate::config::AtlasSimConfig;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<AtlasSimUi>()
            .add_systems(Startup, startup_location)
            .add_systems(UiUpdate, update_ui)
            .add_systems(UiUpdate, update_viewport.after(update_ui))
            .add_systems(UiUpdate, update_location.after(update_viewport));
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
    pub current_panel: Box<dyn SidebarPanel<AtlasSimConfig, Self> + Sync + Send>,
    /// Current mouse cursor coords in world space.
    pub cursor: Option<(u32, u32)>,
}

impl Default for AtlasSimUi {
    fn default() -> Self {
        Self {
            setup_mode: true,
            current_panel: Box::<MainPanelGeneral>::default(),
            cursor: None,
        }
    }
}

impl UiCreator<AtlasSimConfig> for AtlasSimUi {
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
                        open_file_dialog(ui_base, FileDialogMode::ImportSpecial)
                    });
                    button_action_enabled(ui, "Import World State", self.setup_mode, || {
                        open_file_dialog(ui_base, FileDialogMode::Import)
                    });
                    button_action(ui, "Export World State", || {
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
                    button_action_enabled(ui, "Load Configuration", self.setup_mode, || {
                        open_file_dialog(ui_base, FileDialogMode::LoadConfig)
                    });
                    button_action_enabled(ui, "Reset Configuration", self.setup_mode, || {
                        reset_config_clicked(config, self, events)
                    });
                });
                ui.menu_button("Help", |ui| {
                    button_action(ui, "About", || ui_base.about_open = true);
                })
            });
        });
    }

    /// Create sidebar settings for the layer display.
    fn create_layer_view_settings(&self, ui: &mut Ui, ui_base: &mut UiStateBase, events: &mut EventStruct) {
        // Layer visibility dropdown.
        // NOTE: `ui.horizontal_wrapped()` respects `ui.end_row()` used internally by a `SidebarControl`.
        ui.horizontal(|ui| {
            let old = ui_base.current_layer;
            let selection =
                SidebarEnumDropdown::new(ui, "Layer", &mut ui_base.current_layer).show(None);
            SidebarEnumDropdown::post_show(selection, &mut ui_base.current_layer);
            // Trigger layer change event as needed.
            if old != ui_base.current_layer {
                events.viewed_layer_changed = Some(ui_base.current_layer);
            }
            let old = ui_base.current_overlay;
            let selection =
                SidebarEnumDropdown::new(ui, "Overlay", &mut ui_base.current_overlay).show(None);
            SidebarEnumDropdown::post_show(selection, &mut ui_base.current_overlay);
            // Trigger overlay change event as needed.
            if old != ui_base.current_overlay {
                events.viewed_overlay_changed = Some(ui_base.current_overlay);
            }
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
                changed |= button_action(ui, "Scenario", || {
                    self.current_panel = Box::<MainPanelScenario>::default();
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
}

#[derive(Component)]
struct LocationText;

/// Startup system
///
/// Create the top-left location text.
fn startup_location(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection {
                value: "Location: ".to_string(),
                style: TextStyle::default(),
            },
            TextSection {
                value: "-".to_string(),
                style: TextStyle::default(),
            },
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        LocationText,
    ));
}

/// Update system
///
/// Update the string with current mouse coords (mapped to in-map coords).
fn update_location(
    mut ui_state: ResMut<AtlasSimUi>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    map: Query<&Transform, With<CurrentWorldModel>>,
    mut text: Query<&mut Text, With<LocationText>>,
    config: Res<AtlasSimConfig>,
) {
    // Get query results.
    let (camera, camera_transform) = camera.single();
    let window = window.single();
    let map = map.single();
    let mut text = text.single_mut();
    // Update text.
    text.sections[1].value = match ui_state.cursor {
        Some(x) => format!("{}, {}", x.0, x.1),
        None => "-".to_string(),
    };
    // Check if the mouse cursor is inside the window.
    let Some(cursor_position) = window.cursor_position() else {
        ui_state.cursor = None;
        return;
    };
    // Raycast from the camera.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        ui_state.cursor = None;
        return;
    };
    let Some(distance) = ray.intersect_plane(map.translation, Plane3d::new(*map.up())) else {
        ui_state.cursor = None;
        return;
    };
    // Get the coords.
    let coords = ray.get_point(distance);
    let coords = config.world_to_map((coords.x, coords.y));
    ui_state.cursor = Some(coords);
}

/// A visible map overlay.
#[derive(Component)]
pub struct MapOverlay;
