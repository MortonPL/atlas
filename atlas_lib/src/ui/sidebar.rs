use bevy_egui::egui::{self, Grid, Ui, WidgetText};
use std::{marker::PhantomData, ops::RangeInclusive};

use crate::{base::events::EventStruct, domain::map::MapDataLayer, ui::UiEditableEnum};

const NO_HINT_MESSAGE: &str = "PLEASE ADD A HINT";

/// A sidebar page/panel.
pub trait SidebarPanel<C, U>: SidebarPanelCloneHax<C, U> {
    /// Get panel heading.
    /// NOTE: Must be a unique string!
    fn get_heading(&self) -> &'static str;

    /// Get layer that should be displayed with this panel.
    fn get_layer(&self) -> MapDataLayer;

    /// Create a config UI for this panel. Nothing shown by default.
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut C) {}

    /// Create extra UI before the config UI. Nothing shown by default.
    fn extra_ui_pre(&mut self, _ui: &mut Ui, _config: &mut C, _ui_state: &mut U, _events: &mut EventStruct) {}

    /// Create extra UI after the config UI. Nothing shown by default.
    fn extra_ui(&mut self, _ui: &mut Ui, _config: &mut C, _ui_state: &mut U, _events: &mut EventStruct) {}

    /// Create UI for this panel.
    fn show(&mut self, ui: &mut Ui, config: &mut C, ui_state: &mut U, events: &mut EventStruct) {
        self.extra_ui_pre(ui, config, ui_state, events);
        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            self.make_ui(ui, config);
        });
        self.extra_ui(ui, config, ui_state, events);
    }
}

/// A hack to allow trait objects be clonable. https://stackoverflow.com/a/30353928
pub trait SidebarPanelCloneHax<C, U> {
    fn clone_box(&self) -> Box<dyn SidebarPanel<C, U>>;
}

impl<T, C, U> SidebarPanelCloneHax<C, U> for T
where
    T: 'static + Clone + SidebarPanel<C, U>,
{
    fn clone_box(&self) -> Box<dyn SidebarPanel<C, U>> {
        Box::new(self.clone())
    }
}

/// Something that can be edited via the sidebar UI.
pub trait MakeUi {
    fn make_ui(&mut self, ui: &mut Ui);
}

/// A sidebar UI control w/ label that edits a value in a specific way.
/// Designed to be placed inside a two column grid.
pub trait SidebarControl<'u, 'v, T: ?Sized> {
    /// Initialize this control.
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self;

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
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, Num> for SidebarSlider<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut Num) -> Self {
        Self {
            ui,
            inner: egui::DragValue::new(value),
            label: label.into(),
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
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, [Num]> for SidebarSliderN<'u, 'v, [Num]>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, values: &'v mut [Num]) -> Self {
        Self {
            ui,
            inners: values.iter_mut().map(|v| egui::DragValue::new(v)).collect(),
            label: label.into(),
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
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> SidebarControl<'u, 'v, Num> for SidebarSliderRandom<'u, 'v, Num>
where
    Num: egui::emath::Numeric,
    rand::distributions::Standard: rand::distributions::Distribution<Num>,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut Num) -> Self {
        let new = Self {
            ui,
            inner: egui::DragValue::new(value),
            label: label.into(),
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

/// A textbox for a string value.
pub struct SidebarTextbox<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::TextEdit<'v>,
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, T: 'v> SidebarControl<'u, 'v, T> for SidebarTextbox<'u, 'v, T>
where
    &'v mut String: From<&'v mut T>,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        let string = Into::<&mut String>::into(value);
        Self {
            ui,
            inner: egui::TextEdit::singleline(string),
            label: label.into(),
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

/// A checkbox for a boolean value.
pub struct SidebarCheckbox<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::Checkbox<'v>,
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, T: 'v> SidebarControl<'u, 'v, T> for SidebarCheckbox<'u, 'v, T>
where
    &'v mut bool: From<&'v mut T>,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            inner: egui::Checkbox::without_text(value.into()),
            label: label.into(),
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

pub struct SidebarEntityLink<'u, 'v, T> {
    ui: &'u mut Ui,
    label: WidgetText,
    value: WidgetText,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T: 'v> SidebarControl<'u, 'v, T> for SidebarEntityLink<'u, 'v, T>
where
    &'v mut T: core::fmt::Debug,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            label: label.into(),
            value: format!("{:?}", value).into(),
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        let result = self.ui.button(self.value).on_hover_text_at_pointer(hint);
        self.ui.end_row();
        result.clicked() as usize
    }
}

/// A fake control for a header.
pub struct SidebarHeader<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v T,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarHeader<'u, 'v, T>
where
    &'v T: Into<bevy_egui::egui::RichText>,
{
    fn new(ui: &'u mut Ui, _label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self { ui, value }
    }

    fn show(self, _hint: Option<&str>) -> usize {
        self.ui.heading(self.value);
        self.ui.end_row();
        0
    }
}

/// A color picker for an RGB value.
pub struct SidebarColor<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut [u8; 3],
    label: WidgetText,
    __: PhantomData<T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarColor<'u, 'v, T>
where
    T: AsRgb,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            value: value.as_rgb(),
            label: label.into(),
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        self.ui
            .color_edit_button_srgb(self.value)
            .on_hover_text_at_pointer(hint);
        self.ui.end_row();
        0
    }
}

trait AsRgb {
    fn as_rgb(&mut self) -> &mut [u8; 3];
}

impl AsRgb for [u8; 3] {
    fn as_rgb(&mut self) -> &mut [u8; 3] {
        self
    }
}

/// A dropdown for an enum. The enum must have [`UiEditableEnum`] trait.
pub struct SidebarEnumDropdown<'u, 'v, T> {
    ui: &'u mut Ui,
    selection: usize,
    label: WidgetText,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarEnumDropdown<'u, 'v, T>
where
    T: UiEditableEnum,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        let selection = value.self_as_index();
        Self {
            ui,
            selection,
            label: label.into(),
            __: PhantomData,
        }
    }

    fn show(mut self, hint: Option<&str>) -> usize {
        let hint = hint.unwrap_or(NO_HINT_MESSAGE);
        let text = self.label.text().to_string();
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        egui::ComboBox::new(text, "")
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
    label: WidgetText,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarEnumSection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            value,
            label: label.into(),
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        self.ui
            .heading(self.label.text())
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
    fn inner_show(ui: &mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> usize {
        let result = SidebarEnumDropdown::new(ui, label, value).show(Some("Select a variant"));
        SidebarEnumDropdown::post_show(result, value);
        value.make_ui(ui);
        0
    }
}

/// A (sub)section (headerless) for an enum with fields. The enum must have [`UiEditableEnum`] and [`MakeUi`] traits.
pub struct SidebarEnumSubsection<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut T,
    label: WidgetText,
    __: PhantomData<&'v T>,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarEnumSubsection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            value,
            label: label.into(),
            __: PhantomData,
        }
    }

    fn show(self, _hint: Option<&str>) -> usize {
        Self::inner_show(self.ui, self.label, self.value);
        0
    }
}

impl<'u, 'v, T> SidebarEnumSubsection<'u, 'v, T>
where
    T: UiEditableEnum + MakeUi,
{
    fn inner_show(ui: &mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> usize {
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
    label: WidgetText,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarStructSection<'u, 'v, T>
where
    T: MakeUi,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            value,
            label: label.into(),
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        self.ui
            .heading(self.label.text())
            .on_hover_text_at_pointer(hint.unwrap_or(NO_HINT_MESSAGE));
        self.ui.end_row();
        self.value.make_ui(self.ui);
        self.ui.end_row();
        0
    }
}

/// A (sub)section for a (headerless) struct with fields. The struct must have [`MakeUi`] trait.
pub struct SidebarStructSubsection<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut T,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarStructSubsection<'u, 'v, T>
where
    T: MakeUi,
{
    fn new(ui: &'u mut Ui, _label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self { ui, value }
    }

    fn show(self, _hint: Option<&str>) -> usize {
        self.value.make_ui(self.ui);
        self.ui.end_row();
        0
    }
}

/// A list of structs. The struct must have [`MakeUi`] trait.
pub struct SidebarStructList<'u, 'v, T> {
    ui: &'u mut Ui,
    value: &'v mut T,
    label: WidgetText,
}

impl<'u, 'v, T> SidebarControl<'u, 'v, T> for SidebarStructList<'u, 'v, T>
where
    T: AsSlice,
    T::Item: MakeUi,
{
    fn new(ui: &'u mut Ui, label: impl Into<WidgetText>, value: &'v mut T) -> Self {
        Self {
            ui,
            value,
            label: label.into(),
        }
    }

    fn show(self, hint: Option<&str>) -> usize {
        self.ui
            .heading(self.label.text())
            .on_hover_text_at_pointer(hint.unwrap_or(NO_HINT_MESSAGE));
        self.ui.end_row();
        for element in self.value.as_slice() {
            element.make_ui(self.ui);
            self.ui.end_row();
        }
        0
    }
}

trait AsSlice {
    type Item;

    fn as_slice(&mut self) -> &mut [Self::Item];
}

impl<T> AsSlice for std::vec::Vec<T> {
    type Item = T;

    fn as_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> AsSlice for [T; 13] {
    type Item = T;

    fn as_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}
