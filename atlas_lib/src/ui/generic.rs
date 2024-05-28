use bevy_egui::egui::{self, Align2, Context, Ui, WidgetText};

/// An enum that can be changed via dropdown.
pub trait UiEditableEnum {
    const LEN: usize;

    fn self_as_index(&self) -> usize;

    fn index_as_self(&self, idx: usize) -> Self;

    fn index_to_str(idx: usize) -> &'static str;

    fn self_as_str(&self) -> &'static str {
        Self::index_to_str(self.self_as_index())
    }
}

/// Shorthand for `ui.button(text).clicked()`.
pub fn button(ui: &mut Ui, text: impl Into<WidgetText>) -> bool {
    ui.button(text).clicked()
}

/// [`button`] that executes a function when clicked or returns default value when not.
/// Additionally, [`bevy_egui::egui::Ui::close_menu`] is called.
pub fn button_action<F, T>(ui: &mut Ui, text: impl Into<WidgetText>, fun: F) -> T
where
    F: FnOnce() -> T,
    T: Default,
{
    if ui.button(text).clicked() {
        ui.close_menu();
        fun()
    } else {
        T::default()
    }
}

/// Show a popup window.
pub fn window<F, T>(ctx: &Context, title: &str, open: &mut bool, add_content: F)
where
    F: FnOnce(&mut Ui) -> T,
{
    egui::Window::new(title)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .open(open)
        .collapsible(false)
        .movable(false)
        .show(ctx, add_content);
}
