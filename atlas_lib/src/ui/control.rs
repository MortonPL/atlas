use std::{marker::PhantomData, ops::RangeInclusive};

use bevy_egui::egui::{self, Ui};

pub trait UiControl<'u, 'v, T: ?Sized, TRange> {
    fn new(ui: &'u mut Ui, label: &'static str, value: &'v mut T) -> Self;

    fn show(self, hint: Option<&str>);
}

/// A Slider/TextBox combo for a numerical value.
pub struct UiSlider<'u, 'v, T> {
    ui: &'u mut Ui,
    inner: egui::DragValue<'v>,
    label: &'static str,
    __: PhantomData<T>,
}

impl<'u, 'v, Num> UiControl<'u, 'v, Num, RangeInclusive<Num>> for UiSlider<'u, 'v, Num>
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

    fn show(self, hint: Option<&str>) {
        let hint = hint.unwrap_or("PLEASE ADD A HINT");
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        self.ui.add(self.inner).on_hover_text_at_pointer(hint);
        self.ui.end_row();
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

impl<'u, 'v, Num> UiControl<'u, 'v, [Num], RangeInclusive<Num>> for UiSliderN<'u, 'v, [Num]>
where
    Num: egui::emath::Numeric,
{
    fn new(ui: &'u mut Ui, label: &'static str, values: &'v mut [Num]) -> Self {
        let mut inners = vec![];
        for value in values.iter_mut() {
            inners.push(egui::DragValue::new(value));
        }
        Self {
            ui,
            inners,
            label,
            __: PhantomData,
        }
    }

    fn show(self, hint: Option<&str>) {
        let hint = hint.unwrap_or("PLEASE ADD A HINT");
        self.ui.label(self.label).on_hover_text_at_pointer(hint);
        for inner in self.inners {
            self.ui.add(inner).on_hover_text_at_pointer(hint);
        }
        self.ui.end_row();
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
