use std::{marker::PhantomData, ops::RangeInclusive};
use bevy_egui::egui::{self, Ui};

use crate::ui::generic::UiEditableEnum;

const NO_HINT_MESSAGE: &str = "PLEASE ADD A HINT";

/// Something that can be edited via the sidebar UI.
pub trait MakeUi {
    fn make_ui(&mut self, ui: &mut Ui);
}

/// A sidebar UI control w/ label that edits a value in a specific way.
/// Designed to be placed inside a two column grid.
pub trait SidebarControl<'u, 'v, T: ?Sized> {
    /// Initialize this control.
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self;

    /// Show this control and handle input. Return a numeric value
    /// which means different things depending on the control.
    fn show(self, hint: Option<&str>) -> usize;

    /// HACK ALERT! Some controls need to handle input in two stages.
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
/// Primary use case is for random seeds.
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

/// A dropdown for an enum. The enum must have [`UiEditableEnum`] trait.
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

/// A section for an enum with fields. The enum must have [`UiEditableEnum`] and [`MakeUi`] traits.
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
        self.ui
            .heading(self.label)
            .on_hover_text_at_pointer(hint.unwrap_or(NO_HINT_MESSAGE));
        self.ui.end_row();
        Self::inner_show(self.ui, self.label, self.value);
        self.ui.end_row();
        0
    }
}

impl<'u, 'v, T> SidebarEnumSection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn inner_show(ui: &mut Ui, label: &'static str, value: &'v mut T) -> usize {
        let result = SidebarEnumDropdown::new(ui, label, value).show(Some("Select a variant"));
        SidebarEnumDropdown::post_show(result, value);
        value.make_ui(ui);
        0
    }
}

/// A section for a struct with fields. The struct must have [`MakeUi`] trait.
pub struct SidebarStructSection<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut T,
    label: &'static str,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarStructSection<'u, 'v, T>
where
    T: MakeUi,
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
        self.ui
            .heading(self.label)
            .on_hover_text_at_pointer(hint.unwrap_or(NO_HINT_MESSAGE));
        self.ui.end_row();
        self.value.make_ui(self.ui);
        self.ui.end_row();
        0
    }
}
