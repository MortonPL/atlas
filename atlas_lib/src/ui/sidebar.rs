use std::{marker::PhantomData, ops::RangeInclusive};

use bevy_egui::egui::{self, Ui};

use crate::ui::generic::UiEditableEnum;

const NO_HINT_MESSAGE: &str = "PLEASE ADD A HINT";

pub trait MakeUi {
    fn make_ui(&mut self, ui: &mut Ui) -> Vec<usize>;
}

pub trait SidebarControl<'u, 'v, T: ?Sized> {
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self;

    fn show(self, hint: Option<&str>) -> usize;
}

/// A Slider/TextBox combo for a numerical value.
pub struct SidebarSlider<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::DragValue<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, Num> for SidebarSlider<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut Num) -> Self {
        Self {
            ui,
            inner: egui::DragValue::new(value),
            label,
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        self.ui.add(self.inner).on_hover_text_at_pointer(hint);
        self.ui.end_row();
        0
    }
}

impl<'u, 'v, Num> SidebarSlider<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
{
    pub fn clamp_range(mut self, clamp_range: RangeInclusive<Num>) -> Self {
        self.inner = self.inner.clamp_range(clamp_range);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.inner = self.inner.speed(speed);
        self
    }
}

/// A Slider/TextBox combo for N numerical values.
pub struct SidebarSliderN<'u, 'v, T: ?Sized> {
    ui: &'u mut Ui,
    inners: Vec<egui::DragValue<'v>>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, [Num]> for SidebarSliderN<'u, 'v, [Num]>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: &'static str, values: &'v mut [Num]) -> Self {
        Self {
            ui,
            inners: values.iter_mut().map(|v| egui::DragValue::new(v)).collect(),
            label,
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        self.ui.horizontal(|ui| {
            for inner in self.inners {
                ui.add(inner).on_hover_text_at_pointer(hint);
            }
        });
        self.ui.end_row();
        0
    }
}

impl<'u, 'v, Num> SidebarSliderN<'u, 'v, [Num]>
where
    Num: egui::emath::Numeric,
{
    pub fn clamp_range(mut self, clamp_range: RangeInclusive<Num>) -> Self {
        self.inners = self
            .inners
            .into_iter()
            .map(|x| x.clamp_range(clamp_range.clone()))
            .collect();
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.inners = self.inners.into_iter().map(|x| x.speed(speed)).collect();
        self
    }
}

/// A Slider/TextBox combo for a numerical value with an extra button for a random value.
pub struct SidebarSliderRandom<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::DragValue<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, Num> for SidebarSliderRandom<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut Num) -> Self {
        let inner = egui::DragValue::new(value);
        Self {
            ui,
            inner,
            label,
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        let mut clicked = 0;
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        self.ui.horizontal(|ui| {
            ui.add(self.inner).on_hover_text_at_pointer(hint);
            clicked = ui.button("Random").clicked() as usize;
        });
        self.ui.end_row();
        clicked
    }
}

impl<'u, 'v, Num> SidebarSliderRandom<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
{
    pub fn clamp_range(mut self, clamp_range: RangeInclusive<Num>) -> Self {
        self.inner = self.inner.clamp_range(clamp_range);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.inner = self.inner.speed(speed);
        self
    }
}

/// A dropdown for an enum.
pub struct SidebarEnumDropdown<'u, 'v, T> {
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
