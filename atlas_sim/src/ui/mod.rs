mod internal;
mod panel_init;
mod panel_sim;

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
    config::{sim::AtlasSimConfig, AtlasConfig},
    domain::{graphics::CurrentWorldModel, map::MapDataOverlay},
    ui::{
        button, button_action, button_action_enabled,
        sidebar::{SidebarControl, SidebarEnumDropdown, SidebarPanel},
        window,
    },
};
use bevy_mod_picking::{events::Pointer, prelude::*};
use internal::{reset_config_clicked, reset_panel_clicked, FileDialogHandler};
use panel_init::*;
use panel_sim::*;

use crate::sim::{
    polity::Polity,
    region::{Region, RegionUi},
    ui::PolityUi,
    SimControl, SimMapData,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<AtlasSimUi>()
            .add_systems(Startup, startup_location)
            .add_systems(
                UiUpdate,
                (
                    update_ui,
                    update_viewport,
                    update_location,
                    update_click_location,
                    update_selection.run_if(on_event::<UpdateSelectionEvent>()),
                )
                    .chain(),
            )
            .add_event::<UpdateSelectionEvent>()
            .add_systems(UiUpdate, update_selection_data);
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
    mut sim_control: ResMut<SimControl>,
    mut events: ResMut<EventStruct>,
    mut exit: EventWriter<AppExit>,
    window: Query<&Window>,
) {
    if !window.single().focused {
        return;
    }
    ui_state.sim_control = sim_control.clone();
    ui_state.create_ui(
        contexts.ctx_mut(),
        &mut config,
        &mut ui_base,
        &mut events,
        &mut exit,
    );
    sim_control.set_if_neq(ui_state.sim_control.clone());
}

#[derive(Resource)]
struct Selection {
    pub entity: Entity,
    pub polity: Option<PolityUi>,
    pub region: Option<RegionUi>,
}

#[derive(Resource)]
struct AtlasSimUi {
    /// Is the simulation not running yet? Can we still make changes to the configuration?
    pub setup_mode: bool,
    /// SimControl copy.
    pub sim_control: SimControl,
    /// Currently viewed sidebar panel.
    pub current_panel: Box<dyn SidebarPanel<AtlasSimConfig, Self> + Sync + Send>,
    /// Current mouse cursor coords in world space.
    pub cursor: Option<(u32, u32)>,
    /// Pretend that the current panel has changed this frame.
    pub force_changed: bool,
    pub selection: Option<Selection>,
}

impl Default for AtlasSimUi {
    fn default() -> Self {
        Self {
            setup_mode: true,
            sim_control: default(),
            current_panel: Box::<MainPanelGeneral>::default(),
            cursor: None,
            force_changed: false,
            selection: None,
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

    /// Create sidebar settings for the layer display and time control.
    fn create_layer_view_settings(
        &mut self,
        ui: &mut Ui,
        ui_base: &mut UiStateBase,
        events: &mut EventStruct,
    ) {
        // Layer visibility dropdown.
        // NOTE: `ui.horizontal_wrapped()` respects `ui.end_row()` used internally by a `SidebarControl`.
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let label = match (self.setup_mode, self.sim_control.paused) {
                    (true, true) => "Start",
                    (true, false) => "Resume",
                    (false, true) => "Resume",
                    (false, false) => "Pause",
                };
                ui.add_enabled_ui(!self.setup_mode, |ui| {
                    button_action(ui, label, || {
                        self.sim_control.paused = !self.sim_control.paused;
                    });
                    ui.label("Speed");
                    ui.add(
                        egui::DragValue::new(&mut self.sim_control.speed)
                            .prefix("x")
                            .clamp_range(0.0..=60.0),
                    );
                    ui.label("Date:");
                    ui.label(self.sim_control.current_time_to_string());
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                let old = ui_base.current_layer;
                let selection = SidebarEnumDropdown::new(ui, "Layer", &mut ui_base.current_layer).show(None);
                SidebarEnumDropdown::post_show(selection, &mut ui_base.current_layer);
                // Trigger layer change event as needed.
                if old != ui_base.current_layer {
                    events.viewed_layer_changed = Some(ui_base.current_layer);
                }
                ui_base.overlay_window_open |= button(ui, "Overlays");
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
            if self.setup_mode {
                egui::menu::bar(ui, |ui| {
                    tab!("General", MainPanelGeneral, ui);
                    tab!("Scenario", MainPanelScenario, ui);
                    tab!("Rules (Misc)", MainPanelRulesMisc, ui);
                    tab!("Rules (Economy)", MainPanelRulesEco, ui);
                });
                egui::menu::bar(ui, |ui| {
                    tab!("Rules (Tech)", MainPanelRulesTech, ui);
                    tab!("Rules (Culture)", MainPanelRulesCult, ui);
                    tab!("Rules (City)", MainPanelRulesCity, ui);
                    tab!("Climate", MainPanelClimate, ui);
                });
            } else {
                egui::menu::bar(ui, |ui| {
                    tab!("Selected", InfoPanelMisc, ui);
                    tab!("Polity", InfoPanelPolity, ui);
                    tab!("Economy", InfoPanelEconomy, ui);
                    tab!("Science", InfoPanelScience, ui);
                    tab!("Culture", InfoPanelCulture, ui);
                });
            }
            if changed || self.force_changed {
                self.force_changed = false;
                let layer = self.current_panel.get_layer();
                events.viewed_layer_changed = Some(layer);
                ui_base.current_layer = layer;
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
    ui_state.cursor = config.world_to_map((coords.x, coords.y));
}

#[derive(Event)]
pub struct UpdateSelectionEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for UpdateSelectionEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        UpdateSelectionEvent(event.target)
    }
}

fn update_click_location(
    mut ui_state: ResMut<AtlasSimUi>,
    extras: Res<SimMapData>,
    config: Res<AtlasSimConfig>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.just_released(MouseButton::Left) && !ui_state.setup_mode {
        if let Some(cursor) = ui_state.cursor {
            let i = config.map_to_index(cursor) as usize;
            if let Some(entity) = extras.tile_owner[i] {
                ui_state.selection = Some(Selection {
                    entity,
                    polity: None,
                    region: None,
                });
            }
        }
    }
}

/// Update system
///
/// Update the selection if the user clicked.
fn update_selection(mut ui_state: ResMut<AtlasSimUi>, mut event: EventReader<UpdateSelectionEvent>) {
    let event = if let Some(event) = event.read().next() {
        event
    } else {
        return;
    };
    ui_state.selection = Some(Selection {
        entity: event.0,
        polity: None,
        region: None,
    });
}

/// Update system
///
/// Update data of the current selection.
fn update_selection_data(
    mut ui_state: ResMut<AtlasSimUi>,
    polities: Query<&Polity>,
    regions: Query<&Region>,
    config: Res<AtlasSimConfig>,
) {
    let selection = if let Some(selection) = &mut ui_state.selection {
        selection
    } else {
        return;
    };
    let polity = if let Ok(region) = regions.get(selection.entity) {
        selection.region = Some(region.into_ui(&config));
        Some(region.polity)
    } else {
        None
    };
    if let Ok(polity) = polities.get(polity.unwrap_or(selection.entity)) {
        selection.polity = Some(polity.into_ui(&config));
    }
}

/// A visible map overlay.
#[derive(Component)]
pub struct MapOverlay {
    pub overlay: MapDataOverlay,
}

impl MapOverlay {
    pub fn new(overlay: MapDataOverlay) -> Self {
        Self { overlay }
    }
}
