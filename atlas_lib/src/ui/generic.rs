use bevy_egui::egui::{Ui, WidgetText};

pub trait UiEditableEnum {
    const LEN: usize;

    fn self_as_index(&self) -> usize;

    fn index_as_self(&self, idx: usize) -> Self;

    fn index_to_str(idx: usize) -> &'static str;

    fn self_as_str(&self) -> &'static str {
        Self::index_to_str(self.self_as_index())
    }
}

pub fn button(ui: &mut Ui, text: impl Into<WidgetText>) -> bool {
    ui.button(text).clicked()
}

pub fn button_action<F, T>(ui: &mut Ui, text: impl Into<WidgetText>, fun: F) -> T
where
    F: FnOnce() -> T,
    T: Default,
{
    if ui.button(text).clicked() {
        fun()
    } else {
        T::default()
    }
}
