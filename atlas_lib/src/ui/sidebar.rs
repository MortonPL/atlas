use std::{marker::PhantomData, ops::RangeInclusive};

use bevy_egui::egui::{self, Ui};

use crate::ui::generic::UiEditableEnum;

const NO_HINT_MESSAGE: &str = "PLEASE ADD A HINT";

pub trait MakeUi {
    fn make_ui(&mut self, ui: &mut Ui);
}

pub trait SidebarControl<'u, 'v, T: ?Sized> {
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self;

    fn show(self, hint: Option<&str>) -> usize;

    fn post_show(_result: usize, _value: &'v mut T) {}
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
    rand::distributions::Standard: rand::distributions::Distribution<Num>,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut Num) -> Self {
        let new = Self {
            ui,
            inner: egui::DragValue::new(value),
            label,
            __: PhantomData,
        };
        new
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

    fn post_show(result: usize, value: &'v mut Num) {
        if result != 0 {
            *value = rand::random();
        }
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

/// A checkbox for a boolean value.
pub struct SidebarCheckbox<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::Checkbox<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, T: 'v> SidebarControl<'u, 'v, T> for SidebarCheckbox<'u, 'v, T>
where
    &'v mut bool: From<&'v mut T>,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self {
        Self {
            ui,
            inner: egui::Checkbox::without_text(value.into()),
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

    fn post_show(result: usize, value: &'v mut T) {
        if value.self_as_index() != result {
            *value = value.index_as_self(result);
        }
    }
}

/// A section for an enum with fields.
pub struct SidebarEnumSection<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut T,
    label: &'static str,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarEnumSection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self {
        Self {
            ui,
            value,
            label,
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let response = egui::CollapsingHeader::new(egui::RichText::new(self.label).heading())
            .default_open(true)
            .show(self.ui, |ui| {
                egui::Grid::new(format!("{}_section_grid", self.label))
                    .show(ui, |ui| Self::inner_show(ui, self.label, self.value))
            });
        self.ui.end_row();
        response
            .header_response
            .on_hover_text_at_pointer(hint.unwrap_or(NO_HINT_MESSAGE));
        0
    }
}

impl<'u, 'v, T> SidebarEnumSection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn inner_show(ui: &mut Ui, label: &'static str, value: &'v mut T) -> usize {
        let result = SidebarEnumDropdown::new(ui, label, value).show(None); // TODO
        SidebarEnumDropdown::post_show(result, value);
        value.make_ui(ui);
        0
    }
}
