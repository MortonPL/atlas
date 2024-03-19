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

pub fn button<F, T>(ui: &mut Ui, text: impl Into<WidgetText>, fun: F) -> T where F: FnOnce() -> T, T: Default {
    if ui.button(text).clicked() {
        fun()
    } else {
        T::default()
    }
}





/*

/// A dropdown for an enum.
pub struct UiEnumDropdown<'u, 'v, T> {
    ui: &'u mut Ui,
    selection: usize,
    label: &'static str,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarEnumDropdown<'u, 'v, T>
where
    T: UiEditableEnum,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self {
        let selection = value.self_as_index();
        Self {
            ui,
            selection,
            label,
            __: PhantomData,
        }
    }

    fn show(mut self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        egui::ComboBox::new(self.label, "")
            .show_index(self.ui, &mut self.selection, T::LEN, T::index_to_str)
            .on_hover_text_at_pointer(hint);
        self.ui.end_row();
        self.selection
    }
}

*/
