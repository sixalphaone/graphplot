use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{Edge, Node, NodeId, Port, RectStyling, Style};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlotConfig {
    beta: bool,
    layout: Layout,
    lines: Lines,
    orientation: Orientation,
    spacing: Spacing,
    style: Style,
    url: String,
}
impl PlotConfig {
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
    pub fn layout(mut self, new_layout: Layout) -> Self {
        self.layout = new_layout;
        self
    }
    /// Sets the line style of the final plot.
    pub fn lines(mut self, new_lines: Lines) -> Self {
        self.lines = new_lines;
        self
    }
    /// Sets the orientation of the final plot.
    pub fn orientation(mut self, new_orientation: Orientation) -> Self {
        self.orientation = new_orientation;
        self
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
}
impl Default for PlotConfig {
    fn default() -> Self {
        Self::mode(true)
    }
}
impl PlotConfig {
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
    /// Custom theme suitable for flowgraphs. For best result use with custom `create_edge(..)` function (sets headport,
    /// tailport and floating label):
    /// ```rust
    /// let (create_edge_fn, theme) = Theme::flowgraph(true);
    /// let edge = create_edge_fn(node_a, node_b).label("a - b");
    /// ```
    ///
    /// # Adjustments
    /// - Vertical spacing: 200
    /// - Node:
    ///     - Margin: -1
    ///     - Width: 60
    /// - Node (highlighted):
    ///     - Margin: -1
    pub fn flowgraph(dark: bool) -> (impl Fn(NodeId, NodeId) -> Edge, Self) {
        let create_edge_fn = |from, to| Edge::from(from, to).headport(Port::West).tailport(Port::East).floating();
        (
            create_edge_fn,
            PlotConfig::mode(dark)
                .orientation(Orientation::Lr)
                .spacing(|spacing| spacing.vertical(Space::Static(200.0)))
                .style(|theme| {
                    theme
                        .edge(|e| e.line(|l| l.arrowsize(7)))
                        .edge_highlighted(|e| e.line(|l| l.arrowsize(7)))
                        .node(|node| node.margin(-1.0).width(60))
                        .node_highlighted(|node| node.margin(-1.0))
                }),
        )
    }
    /// Custom theme sutiable for trees with 3 different Node-styles:
    /// 1. leaf Node: highlighted Node
    /// 2. internal Node: regular Node
    /// 3. root Node: returned functions:
    ///   ```rust
    ///   let (create_root_fn, theme) = Theme::tree(true);
    ///   let node = create_root_fn().label("A");
    ///   ```
    ///
    /// # Adjustments
    /// - Lines: `Lines::Line`
    /// - Spacing (vertical): 16
    /// - Node margin: -0.3
    /// - Colors
    pub fn tree(dark: bool) -> (impl Fn() -> Node, Self) {
        let theme = PlotConfig::mode(dark)
            .spacing(|s| s.vertical(Space::Static(16.0)))
            .lines(Lines::Line)
            .style(|t| t.node(|n| n.margin(-0.3)).node_highlighted(|n| n.margin(-0.3)));

        // 1. create root Node-fn:
        let pink_nodestyle = theme
            .get_style()
            .get_node()
            .clone()
            .background_color("#d7bef7")
            .background_opacity(0.3)
            .border_radius(2.0)
            .padding(2.5)
            .frame(|f| f.enable().color("#d7bef7"));
        let create_root_node_fn = move || Node::new().style(pink_nodestyle.clone());

        // return
        (
            create_root_node_fn,
            theme.style(|t| t.node(|n| n.padding(2.0)).node_highlighted(|n| n.padding(2.0))),
        )
    }
}

/// Spesifies the layout of the Plot.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    Circular,
    Forcedirected,
    ForcedirectedScaled,
    #[default]
    Layered,
    Radial,
    Spring,
    Structured,
}

/// Spesifies the type of lines to use.
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lines {
    #[default]
    Spline,
    None,
    Line,
    Curved,
    Polyline,
    /// Draws "technical" lines. For best result set Edge-labels to "floating".
    Technical,
    /// **Beta:** Draws rounded "technical" lines with custom corner-radius (max).
    TechnicalRounded(f32),
}

/// Specifies the orientation of the [`Multigraph`].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Orientation {
    /// Top -> bottom
    #[default]
    Tb,
    /// Bottom -> top
    Bt,
    /// Left -> right
    Lr,
    /// Right -> left
    Rl,
}

/// Specifies the distance between nodes in both horizontal and vertical direction.
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Spacing {
    horizontal: Space,
    vertical: Space,
}
impl Spacing {
    // -- getters
    pub fn get_horizontal(&self) -> Space {
        self.horizontal
    }
    pub fn get_vertical(&self) -> Space {
        self.vertical
    }

    // -- setters
    pub fn horizontal(mut self, spacing_option: Space) -> Self {
        self.horizontal = spacing_option;
        self
    }
    pub fn vertical(mut self, spacing_option: Space) -> Self {
        self.vertical = spacing_option;
        self
    }
}

/// Spesifies whether to automatic adjust spacing, or spesify a spesific one (static).
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Space {
    #[default]
    Auto,
    Static(f32),
}
