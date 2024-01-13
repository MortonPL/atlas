use bevy_egui::egui::{self, Response, Ui};

/// Add a section consisting of a collapsible header and a grid.
pub fn add_section<BodyRet>(
    ui: &mut Ui,
    header: impl Into<String>,
    add_body: impl FnOnce(&mut Ui) -> BodyRet,
) -> Response {
    let header: String = header.into();
    egui::CollapsingHeader::new(egui::RichText::new(header.clone()).heading())
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new(format!("{}_grid", header)).show(ui, add_body);
        })
        .header_response
}
