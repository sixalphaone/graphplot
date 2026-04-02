use core::iter::DoubleEndedIterator;
use std::fmt::Display;
use std::{collections::BTreeMap, fmt::Debug};

use serde::{Deserialize, Serialize};

mod edgestyle;
mod graphstyle;
mod nodestyle;

pub use edgestyle::*;
pub use graphstyle::*;
pub use nodestyle::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Style {
    background_color: String,
    background_opacity: f32,
    fullscreen: bool, // svg only
    edge: EdgeStyle,
    edge_highlighted: EdgeStyle,
    graph: GraphStyle,
    node: NodeStyle,
    node_highlighted: NodeStyle,
    subgraph: GraphStyle,
    utils: StyleUtils,
}
impl Style {
    /// Oppretter en [`Theme`] fra fil.
    pub fn from_file(filename: &str) -> Self {
        let json = std::fs::read_to_string(filename).expect("Error reading json with theme");
        serde_json::from_str(&json).expect("Error parsing json to Theme")
    }
    /// Returnerer mørkt standard Theme.
    pub fn dark() -> Self {
        let json = include_str!("../styles/dark.json");
        serde_json::from_str(json).expect("Error parsing json to Theme (includstr! dark.json)")
    }
    /// Returnerer lyst standard Theme.
    pub fn light() -> Self {
        let json = include_str!("../styles/light.json");
        serde_json::from_str(json).expect("Error parsing json to Theme (includstr! light.json)")
    }

    // -- getters
    pub fn get_background_color(&self) -> &str {
        &self.background_color
    }
    pub fn get_background_opacity(&self) -> f32 {
        self.background_opacity
    }
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }
    pub fn get_edge(&self) -> &EdgeStyle {
        &self.edge
    }
    pub fn get_edge_highlighted(&self) -> &EdgeStyle {
        &self.edge_highlighted
    }
    pub fn get_graph(&self) -> &GraphStyle {
        &self.graph
    }
    pub fn get_node(&self) -> &NodeStyle {
        &self.node
    }
    pub fn get_node_highlighted(&self) -> &NodeStyle {
        &self.node_highlighted
    }
    pub fn get_subgraph(&self) -> &GraphStyle {
        &self.subgraph
    }
    pub fn get_utils(&self) -> &StyleUtils {
        &self.utils
    }

    // -- setter
    /// Adds background-color to the plot.
    pub fn background_color<S: Display>(mut self, color: S) -> Self {
        self.background_color = color.to_string();
        self
    }
    pub fn background_opacity(mut self, opacity: f32) -> Self {
        self.background_opacity = opacity;
        self
    }
    /// Disables SVG fullscreen background color.
    pub fn disable_fullscreen(mut self) -> Self {
        self.fullscreen = false;
        self
    }
    /// Adds a section to <defs>.
    pub fn def<S: Display>(mut self, def: S) -> Self {
        self.utils.defs.push(def.to_string());
        self
    }
    /// Adds a webfont URL to the SVG. E.g: "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;1,100;1,200;1,300;1,400;1,500;1,600;1,700".
    pub fn webfont<S: Display>(mut self, font: S) -> Self {
        self.utils.webfonts.push(font.to_string());
        self
    }

    // -- builders
    pub fn edge(mut self, configure_edge: impl FnOnce(EdgeStyle) -> EdgeStyle) -> Self {
        self.edge = configure_edge(self.edge);
        self
    }
    pub fn edge_highlighted(mut self, configure_edge_highlighted: impl FnOnce(EdgeStyle) -> EdgeStyle) -> Self {
        self.edge_highlighted = configure_edge_highlighted(self.edge_highlighted);
        self
    }
    pub fn graph(mut self, configure_graph: impl FnOnce(GraphStyle) -> GraphStyle) -> Self {
        self.graph = configure_graph(self.graph);
        self
    }
    pub fn node(mut self, configure_node: impl FnOnce(NodeStyle) -> NodeStyle) -> Self {
        self.node = configure_node(self.node);
        self
    }
    pub fn node_highlighted(mut self, configure_node_highlighted: impl FnOnce(NodeStyle) -> NodeStyle) -> Self {
        self.node_highlighted = configure_node_highlighted(self.node_highlighted);
        self
    }
    pub fn subgraph(mut self, configure_subgraph: impl FnOnce(GraphStyle) -> GraphStyle) -> Self {
        self.subgraph = configure_subgraph(self.subgraph);
        self
    }
}
impl Default for Style {
    fn default() -> Self {
        Self::light()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StyleUtils {
    defs: Vec<String>,
    webfonts: Vec<String>,
}
impl StyleUtils {
    // -- getters
    /// Returns every <defs> -section added to theme.
    pub fn get_defs(&self) -> &[String] {
        &self.defs
    }
    /// Returns every webfont-URL (Googl Fonts, Adobe Fonts etc.)
    pub fn get_webfonts(&self) -> &[String] {
        &self.webfonts
    }
}

// -- shared
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FontStyle {
    color: String,
    family: String,
    anchor: TextAnchor,
    opacity: f32,
    size: f32,
    attributes: BTreeMap<String, String>,
}
impl FontStyle {
    // -- getters
    pub fn get_anchor(&self) -> TextAnchor {
        self.anchor
    }
    pub fn get_color(&self) -> &str {
        &self.color
    }
    pub fn get_family(&self) -> &str {
        &self.family
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    pub fn get_size(&self) -> f32 {
        self.size
    }
    pub fn attrs(&self) -> impl DoubleEndedIterator<Item = (&String, &String)> {
        self.attributes.iter()
    }

    // -- setters
    pub fn anchor(mut self, anchor: TextAnchor) -> Self {
        self.anchor = anchor;
        self
    }
    pub fn color<S: Display>(mut self, color: S) -> Self {
        self.color = color.to_string();
        self
    }
    pub fn family<S: Display>(mut self, family: S) -> Self {
        self.family = family.to_string();
        self
    }
    pub fn opacity<I: Into<f64>>(mut self, new_opacity: I) -> Self {
        self.opacity = new_opacity.into() as f32;
        self
    }
    pub fn size<I: Into<f64>>(mut self, new_size: I) -> Self {
        self.size = new_size.into() as f32;
        self
    }
    /// Sets an attribute on the `<text>` element.
    pub fn set<K: Display, V: Display>(mut self, attr: K, value: V) -> Self {
        self.attributes.insert(attr.to_string(), value.to_string());
        self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FrameStyle {
    enabled: bool,
    color: String,
    opacity: f32,
    thickness: f32,
    attributes: BTreeMap<String, String>,
}
impl FrameStyle {
    // -- getters
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn get_color(&self) -> &str {
        &self.color
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    pub fn get_thickness(&self) -> f32 {
        self.thickness
    }
    pub fn attrs(&self) -> impl DoubleEndedIterator<Item = (&String, &String)> {
        self.attributes.iter()
    }

    // -- setters
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }
    pub fn color<S: Display>(mut self, color: S) -> Self {
        self.color = color.to_string();
        self
    }
    pub fn opacity<I: Into<f64>>(mut self, new_opacity: I) -> Self {
        self.opacity = new_opacity.into() as f32;
        self
    }
    pub fn thickness<I: Into<f64>>(mut self, new_thickness: I) -> Self {
        self.thickness = new_thickness.into() as f32;
        self
    }
    /// Sets an attribute on `<rect>` or `<circle>` element.
    pub fn set<K: Display, V: Display>(mut self, attr: K, value: V) -> Self {
        self.attributes.insert(attr.to_string(), value.to_string());
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LineStyle {
    arrowsize: f32,
    color: String,
    opacity: f32,
    thickness: f32,
    attributes: BTreeMap<String, String>,
}
impl LineStyle {
    // -- getters
    pub fn get_arrowsize(&self) -> f32 {
        self.arrowsize
    }
    pub fn get_color(&self) -> &str {
        &self.color
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    pub fn get_thickness(&self) -> f32 {
        self.thickness
    }
    pub fn attrs(&self) -> impl DoubleEndedIterator<Item = (&String, &String)> {
        self.attributes.iter()
    }

    // -- setters
    pub fn arrowsize<I: Into<f64>>(mut self, new_arrowsize: I) -> Self {
        self.arrowsize = new_arrowsize.into() as f32;
        self
    }
    pub fn color<S: Display>(mut self, color: S) -> Self {
        self.color = color.to_string();
        self
    }
    pub fn opacity<I: Into<f64>>(mut self, new_opacity: I) -> Self {
        self.opacity = new_opacity.into() as f32;
        self
    }
    pub fn thickness<I: Into<f64>>(mut self, new_thickness: I) -> Self {
        self.thickness = new_thickness.into() as f32;
        self
    }
    /// Sets an attribute on the `<path>` element.
    pub fn set<K: Display, V: Display>(mut self, attr: K, value: V) -> Self {
        self.attributes.insert(attr.to_string(), value.to_string());
        self
    }
}

#[derive(Copy, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}
impl Debug for TextAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let anchor = match self {
            Self::Start => "start",
            Self::Middle => "middle",
            Self::End => "end",
        };
        write!(f, "{anchor}")
    }
}

// -- shared: traits
pub trait RectStyling {
    // -- getters
    fn get_background_color(&self) -> &str;
    fn get_background_opacity(&self) -> f32;
    fn get_border_radius(&self) -> f32;
    fn get_padding(&self) -> f32;
    fn get_margin(&self) -> f32;
    fn get_height(&self) -> Option<f32>;
    fn get_width(&self) -> Option<f32>;

    // -- builder
    fn background_color<S: Display>(self, color: S) -> Self;
    fn background_opacity<I: Into<f64>>(self, opacity: I) -> Self;
    fn border_radius<I: Into<f64>>(self, radius: I) -> Self;
    fn padding<I: Into<f64>>(self, padding: I) -> Self;
    fn margin<I: Into<f64>>(self, margin: I) -> Self;
    /// Sets minimal height.
    fn height<I: Into<f64>>(self, height: I) -> Self;
    /// Sets minimal width.
    fn width<I: Into<f64>>(self, width: I) -> Self;
}
