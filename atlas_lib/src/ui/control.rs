use std::{marker::PhantomData, ops::RangeInclusive};

use bevy_egui::egui::{self, Ui};

const NO_HINT_MESSAGE: &str = "PLEASE ADD A HINT";

pub trait UiControl<'u, 'v, T: ?Sized> {
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self;

    fn show(self, hint: Option<&str>) -> usize;
}

pub trait UiConfigurableEnum {
    const LEN: usize;

    fn self_as_index(&self) -> usize;

    fn index_as_self(&self, idx: usize) -> Self;

    fn index_to_str(idx: usize) -> &'static str;

    fn self_as_str(&self) -> &'static str {
        Self::index_to_str(self.self_as_index())
    }
}

/// A Slider/TextBox combo for a numerical value.
pub struct UiSlider<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::DragValue<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> UiControl<'u, 'v, Num> for UiSlider<'u, 'v, Num>
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

impl<'u, 'v, Num> UiSlider<'u, 'v, Num>
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
pub struct UiSliderN<'u, 'v, T: ?Sized> {
    ui: &'u mut Ui,
    inners: Vec<egui::DragValue<'v>>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> UiControl<'u, 'v, [Num]> for UiSliderN<'u, 'v, [Num]>
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
        for inner in self.inners {
            self.ui.add(inner).on_hover_text_at_pointer(hint);
        }
        self.ui.end_row();
        0
    }
}

impl<'u, 'v, Num> UiSliderN<'u, 'v, [Num]>
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
pub struct UiSliderRandom<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::DragValue<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> UiControl<'u, 'v, Num> for UiSliderRandom<'u, 'v, Num>
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

impl<'u, 'v, Num> UiSliderRandom<'u, 'v, Num>
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
pub struct UiEnumDropdown<'u, 'v, T> {
    ui: &'u mut Ui,
    selection: usize,
    label: &'static str,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> UiControl<'u, 'v, T> for UiEnumDropdown<'u, 'v, T>
where
    T: UiConfigurableEnum,
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
