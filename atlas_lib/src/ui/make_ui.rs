use bevy_egui::egui::Ui;

pub trait MakeUi {
    fn make_ui(&mut self, ui: &mut Ui);
}
