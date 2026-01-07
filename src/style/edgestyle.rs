use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::LabelAnchor;

use super::{FontStyle, FrameStyle, LineStyle, RectStyling};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EdgeStyle {
    background_color: String,
    background_opacity: f32,
    border_radius: f32,
    labelanchor: LabelAnchor,
    padding: f32,
    margin: f32,
    // -- optional
    height: Option<f32>,
    width: Option<f32>,
    // -- objects
    font: FontStyle,
    labelfont: FontStyle,
    frame: FrameStyle,
    line: LineStyle,
}
impl EdgeStyle {
    // -- getters
    pub fn get_font(&self) -> &FontStyle {
        &self.font
    }
    pub fn get_frame(&self) -> &FrameStyle {
        &self.frame
    }
    pub fn get_labelanchor(&self) -> LabelAnchor {
        self.labelanchor
    }
    pub fn get_labelfont(&self) -> &FontStyle {
        &self.labelfont
    }
    pub fn get_line(&self) -> &LineStyle {
        &self.line
    }

    // -- builders
    pub fn font(mut self, configure_font: impl FnOnce(FontStyle) -> FontStyle) -> Self {
        self.font = configure_font(self.font);
        self
    }
    pub fn frame(mut self, configure_frame: impl FnOnce(FrameStyle) -> FrameStyle) -> Self {
        self.frame = configure_frame(self.frame);
        self
    }
    pub fn labelanchor(mut self, anchor: LabelAnchor) -> Self {
        self.labelanchor = anchor;
        self
    }
    pub fn labelfont(mut self, configure_labelfont: impl FnOnce(FontStyle) -> FontStyle) -> Self {
        self.labelfont = configure_labelfont(self.labelfont);
        self
    }
    pub fn line(mut self, configure_line: impl FnOnce(LineStyle) -> LineStyle) -> Self {
        self.line = configure_line(self.line);
        self
    }
}
impl RectStyling for EdgeStyle {
    // -- getter
    fn get_background_color(&self) -> &str {
        &self.background_color
    }
    fn get_background_opacity(&self) -> f32 {
        self.background_opacity
    }
    fn get_border_radius(&self) -> f32 {
        self.border_radius
    }
    fn get_padding(&self) -> f32 {
        self.padding
    }
    fn get_margin(&self) -> f32 {
        self.margin
    }
    fn get_height(&self) -> Option<f32> {
        self.height
    }
    fn get_width(&self) -> Option<f32> {
        self.width
    }

    // -- builder
    fn background_color<S: Display>(mut self, color: S) -> Self {
        self.background_color = color.to_string();
        self
    }
    fn background_opacity<I: Into<f64>>(mut self, opacity: I) -> Self {
        self.background_opacity = opacity.into() as f32;
        self
    }
    fn border_radius<I: Into<f64>>(mut self, radius: I) -> Self {
        self.border_radius = radius.into() as f32;
        self
    }
    fn padding<I: Into<f64>>(mut self, padding: I) -> Self {
        self.padding = padding.into() as f32;
        self
    }
    fn margin<I: Into<f64>>(mut self, margin: I) -> Self {
        self.margin = margin.into() as f32;
        self
    }
    fn height<I: Into<f64>>(mut self, height: I) -> Self {
        self.height = Some(height.into() as f32);
        self
    }
    fn width<I: Into<f64>>(mut self, width: I) -> Self {
        self.width = Some(width.into() as f32);
        self
    }
}
