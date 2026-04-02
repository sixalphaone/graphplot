use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{FontStyle, FrameStyle, RectStyling};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NodeStyle {
    background_color: String,
    background_opacity: f32,
    border_radius: f32,
    padding: f32,
    margin: f32,
    height: Option<f32>,
    width: Option<f32>,
    // -- objects
    font: FontStyle,
    frame: FrameStyle,
    labelfont: FontStyle,
}
impl NodeStyle {
    // -- getters
    pub fn get_font(&self) -> &FontStyle {
        &self.font
    }
    pub fn get_frame(&self) -> &FrameStyle {
        &self.frame
    }
    pub fn get_labelfont(&self) -> &FontStyle {
        &self.labelfont
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
    pub fn labelfont(mut self, configure_labelfont: impl FnOnce(FontStyle) -> FontStyle) -> Self {
        self.labelfont = configure_labelfont(self.labelfont);
        self
    }
}
impl RectStyling for NodeStyle {
    // -- getters
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
