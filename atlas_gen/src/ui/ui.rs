use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Context, Sense, Align, Response, Ui, WidgetText, Layout},
    EguiContexts,
};
use core::hash::Hash;
use documented::{Documented, DocumentedFields};

use crate::config::{GeneratorConfig, WorldModel, GeneralConfig};

const HEADER_COUNT: usize = 3;
const HEADER_GENERAL_ID: usize = 1;
const HEADER_TOPOGRAPHY_ID: usize = 2;
const HEADER_CLIMATE_ID: usize = 3;

const SIDEBAR_WIDTH: f32 = 200.0;
const TOPBAR_HEIGHT: f32 = 50.0;

pub fn ui_system(config: ResMut<GeneratorConfig>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let scroll_to_header = add_side_panel(ctx);

    add_top_panel(ctx);

    add_central_panel(ctx, config, scroll_to_header);
}

/// Add a navigation side panel with clickable headers.
fn add_side_panel(ctx: &Context) -> usize {
    let mut scroll_to_header = 0;
    egui::SidePanel::left("side_panel")
        .default_width(SIDEBAR_WIDTH)
        .show(ctx, |ui| {
            ui.heading("Navigation");

            if add_nav_header(ui, "General")
            {
                scroll_to_header = HEADER_GENERAL_ID;
            }
            if add_nav_header(ui, "Topography")
            {
                scroll_to_header = HEADER_TOPOGRAPHY_ID;
            }
            if add_nav_header(ui, "Climate")
            {
                scroll_to_header = HEADER_CLIMATE_ID;
            }
        });
    scroll_to_header
}

/// Add a clickable header to the navigation panel.
fn add_nav_header(ui: &mut Ui, header: impl Into<WidgetText>) -> bool {
    ui
        .add(egui::Label::new(header).sense(Sense::click()))
        .clicked()
}

/// Add a top bar with configuration S/L.
fn add_top_panel(ctx: &Context) {
    egui::TopBottomPanel::top("top_panel")
        .default_height(TOPBAR_HEIGHT)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("Atlas Map Generator").size(36.0));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.button(egui::RichText::new("Save Configuration").size(18.0));
                    ui.button(egui::RichText::new("Load Configuration").size(18.0));
                });
            });
        });
}

/// Add the central panel with main GUI.
fn add_central_panel(ctx: &Context, mut config: ResMut<GeneratorConfig>, scroll_to_header: usize) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut headers = Vec::with_capacity(HEADER_COUNT);

            let header = add_collapsible_header(ctx, ui, "header_1", "General", |ui| {
                egui::Grid::new("header_1_grid").show(ui, |ui| {
                    ui.label("World Model").on_hover_text_at_pointer(GeneralConfig::get_field_comment("world_model").unwrap_or_default());
                    egui::ComboBox::from_label("")
                        .selected_text(config.general.world_model.str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut config.general.world_model, WorldModel::Flat, WorldModel::Flat.str());
                            ui.selectable_value(&mut config.general.world_model, WorldModel::Globe, WorldModel::Globe.str())
                        }).response.on_hover_text_at_pointer(GeneralConfig::get_field_comment("world_model").unwrap_or_default());
                    ui.end_row();
                    ui.label("AAAAAAAAAAAAAAAAAAAAAAAAAAAA");
                    ui.label("BBBB");
                    ui.end_row();
                });
            });
            headers.push(header);

            let header = add_collapsible_header(ctx, ui, "header_2", "Topography", |ui| {});
            headers.push(header);

            let header = add_collapsible_header(ctx, ui, "header_3", "Climate", |ui| {});
            headers.push(header);

            if scroll_to_header > 0 {
                headers[scroll_to_header - 1].scroll_to_me(Some(Align::TOP));
            }
        });
    });
}

/// Add a collapsible header to the central panel.
fn add_collapsible_header<BodyRet>(ctx: &Context, ui: &mut Ui, id: impl Hash, header: impl Into<String>, add_body: impl FnOnce(&mut Ui) -> BodyRet) -> Response {
    egui::collapsing_header::CollapsingState::load_with_default_open(
        ctx,
        ui.make_persistent_id(id),
        true,
    )
    .show_header(ui, |ui| {
        ui.add(egui::Label::new(egui::RichText::new(header).heading()))
    })
    .body(|ui| add_body(ui)).0
}
