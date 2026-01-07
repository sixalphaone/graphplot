use std::{
    fmt::{self, Display},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

mod config;
mod edge;
mod multigraph;
mod node;
mod style;
mod subgraph;

const TYPST_PREFIX: &str = "__typst__";

// -- public export
pub mod extras;

pub use config::*;
pub use edge::{ArrowKind, Edge};
pub use multigraph::Multigraph;
pub use node::{Node, NodeShape};
pub use style::*;
pub use subgraph::Subgraph;

// -- types
pub type EdgeId = usize;
pub type NodeId = usize;
pub type SubgraphId = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub enum Port {
    West,
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    Center,
}

/// Represents a plot produced by Graphplot.
#[derive(Clone, Debug)]
pub struct PlotSVG {
    header: String,
    defs: String,
    content: String,
    style: String,
}
impl PlotSVG {
    /// Creates a [`PlotSVG`] from plot-content, -width & -height. Additionally it adds fonts and background-color.
    pub fn from(content: String, width: f32, height: f32, theme: &Style) -> Self {
        let header = format!("<svg viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">");
        let (mut defs, mut style) = (String::new(), String::new());

        // 1. add <defs>
        for section in theme.get_utils().get_defs() {
            defs.push_str(section);
        }

        // 2. add <style>
        // 2.1 web fonts
        for webfont_url in theme.get_utils().get_webfonts() {
            style += &format!("@import url(\"{webfont_url}\");\n");
        }
        // 2.2. background-color
        style += &format!("svg {{ background-color: {}; }}\n", theme.get_background_color());

        // return
        Self { content, defs, header, style }
    }
}
impl PlotSVG {
    /// Returns a complete SVG with sourrounding <svg>-tags and added <style> with fonts and so on.
    /// The `.to_string()` method only returns the content of the plot as an SVG element (<group>). This is useful when
    /// nesting [`Multigraph`]'s and want to this [`Multigraph`] as a Node's object in another.
    pub fn to_complete_svg(&self) -> String {
        format!(
            "{header}\n<defs>{defs}</defs>\n{content}\n<style>\n{style}</style>\n</svg>",
            header = self.header,
            defs = self.defs,
            content = self.content,
            style = self.style
        )
    }
    /// Exporting the complete plot as a SVG to file.
    pub fn save<F: Display>(&self, filename: F) -> Result<()> {
        let mut filepath = filename.to_string();

        // 1. append filextension if not exist
        if !filepath.ends_with(".svg") {
            filepath += ".svg";
        }

        // 2. add parent folders (if not exist)
        let path = Path::new(&filepath);
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        // 3. get complete SVG
        let svg = self.to_complete_svg();

        // 4. save to file
        std::fs::write(filepath, svg).context("Error saving Graphplot to file")
    }
}
impl fmt::Display for PlotSVG {
    /// Returns the content of the plot as an SVG element (<group>). This is useful when nesting [`Multigraph`]'s and
    /// want to this [`Multigraph`] as a Node's object in another.
    ///
    /// Content is without <svg>-tag and <style>.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
