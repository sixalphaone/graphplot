use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{RectStyling, Style};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Options {
    beta: bool,
    layout: Layout,
    lines: Lines,
    orientation: Orientation,
    spacing: Spacing,
    style: Style,
    url: String,
}
impl Options {
    /// Read from JSON-file.
    pub fn from_json(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path).context("Error reading JSON with plot Config")?;
        serde_json::from_str(&json).context("Error parsing JSON to plot Config")
    }

    // -- getters
    pub fn get_layout(&self) -> Layout {
        self.layout
    }
    pub fn get_lines(&self) -> Lines {
        self.lines
    }
    pub fn get_orientation(&self) -> Orientation {
        self.orientation
    }
    pub fn get_spacing(&self) -> Spacing {
        self.spacing
    }
    pub fn get_style(&self) -> &Style {
        &self.style
    }
    pub fn get_url(&self) -> String {
        // check: stable or beta channel?
        let channel = match self.beta {
            true => "beta",
            false => "stable",
        };
        // return
        match self.url.ends_with("/") {
            true => format!("{}{channel}", self.url),
            false => format!("{}/{channel}", self.url),
        }
    }

    // -- setters
    /// Using the beta channel in the API.
    pub fn beta(mut self) -> Self {
        self.beta = true;
        self
    }
    /// Sets the layout style of the final plot.
    ///
    /// # Notes
    /// - `Layout` must be specified before `Lines`.
    pub fn layout(mut self, new_layout: Layout) -> Self {
        self.layout = new_layout;

        // check: already set Lines::Techincal(..)?
        self.lines = if let Lines::Technical(radius) = self.lines {
            match new_layout {
                Layout::Diagram | Layout::DiagramBalanced => Lines::Technical(radius),
                _ => Lines::Spline,
            }
        } else {
            Lines::Spline
        };

        // return
        self
    }
    /// Sets the orientation of the final plot.
    pub fn orientation(mut self, new_orientation: Orientation) -> Self {
        self.orientation = new_orientation;
        self
    }
    /// Sets the corner radius of orthogonal Edges (only Diagrams).
    pub fn radius<R: Into<f64>>(mut self, radius: R) -> Self {
        // check: not Diagram? -> ignore
        if ![Layout::Diagram, Layout::DiagramBalanced].contains(&self.layout) {
            return self;
        }
        self.lines = Lines::Technical(radius.into() as f32);
        self
    }
    /// Scales every `Style` element.
    pub fn scale<S: Into<f64>>(self, scale: S) -> Self {
        let scale_f64 = scale.into();
        self.style(|s| s.scale(scale_f64)).spacing(|s| s.scale(scale_f64))
    }
    /// **Don't use**: Only relevant for Graphplot developers.
    pub fn dev(mut self) -> Self {
        self.url = "http://0.0.0.0:9000".to_string();
        self
    }

    // -- builders
    /// Specify vertical and horizontal spacing in the final plot. If `Orientation::Lr` or `Orientation::Rl` is used
    /// then vertical and horizontal spacing is swapped.
    pub fn spacing(mut self, configure_spacing: impl FnOnce(Spacing) -> Spacing) -> Self {
        self.spacing = configure_spacing(self.spacing);
        self
    }
    /// Customize style of the final plot (e.g colors, font, font-size, borders, arrowsize, edge-thickness etc).
    pub fn style(mut self, configure_style: impl FnOnce(Style) -> Style) -> Self {
        self.style = configure_style(self.style);
        self
    }

    // -- private
    /// Sets the line style of the final plot.
    fn _lines(mut self, new_lines: Lines) -> Self {
        self.lines = new_lines;
        self
    }
}
impl Default for Options {
    fn default() -> Self {
        Self::mode(true)
    }
}
impl Options {
    fn mode(dark: bool) -> Self {
        Self {
            beta: false,
            layout: Layout::default(),
            lines: Lines::default(),
            orientation: Orientation::default(),
            spacing: Spacing::default(),
            style: if dark { Style::dark() } else { Style::light() },
            url: "https://api.graphplot.io/".into(),
        }
    }
    /// Default light theme suitable for print (disabled background colors).
    pub fn print() -> Self {
        let config = Self::mode(false);
        config.style(|s| s.background_color("#ffffffff").graph(|g| g.background_color("#ffffffff")))
    }
    /// Default light theme.
    pub fn light() -> Self {
        Self::mode(false)
    }
    /// Default dark theme.
    pub fn dark() -> Self {
        Self::mode(true)
    }
}

/// Spesifies the layout of the Plot.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    // -- graphviz
    Circular,
    Forcedirected,
    ForcedirectedScaled,
    Layered,
    Radial,
    Spring,
    // -- elk
    #[default]
    Diagram,
    DiagramBalanced,
}

/// Spesifies the type of lines to use.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lines {
    Spline,
    None,
    Line,
    Curved,
    Polyline,
    /// Draws rounded "technical" lines with custom corner-radius.
    Technical(f32),
}
impl Default for Lines {
    fn default() -> Self {
        Self::Technical(5.0)
    }
}

/// Specifies the orientation of the [`Multigraph`].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Orientation {
    #[default]
    Undefined,
    /// Top -> bottom
    Down,
    /// Bottom -> top
    Up,
    /// Left -> right
    Right,
    /// Right -> left
    Left,
}

/// Specifies the distance between nodes in both horizontal and vertical direction.
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Spacing {
    nodes: Space,
    layers: Space,
}
impl Spacing {
    // -- getters
    pub fn get_nodes(&self) -> Space {
        self.nodes
    }
    pub fn get_layers(&self) -> Space {
        self.layers
    }

    // -- setters
    pub fn layers<S: Into<f64>>(mut self, space: S) -> Self {
        self.layers = Space::Static(space.into());
        self
    }
    pub fn nodes<S: Into<f64>>(mut self, space: S) -> Self {
        self.nodes = Space::Static(space.into());
        self
    }
    pub fn scale<S: Into<f64>>(mut self, scale: S) -> Self {
        let scale_f64 = scale.into();
        if let Space::Static(layers) = self.layers {
            self.layers = Space::Static(layers * scale_f64);
        }
        if let Space::Static(nodes) = self.nodes {
            self.nodes = Space::Static(nodes * scale_f64);
        }
        // return
        self
    }
}

/// Spesifies whether to automatic adjust spacing, or spesify a spesific one (static).
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Space {
    #[default]
    Auto,
    Static(f64),
}
