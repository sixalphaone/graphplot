use std::{
    fmt::{self, Display},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "png", feature = "pdf"))]
use include_dir::{Dir, include_dir};
#[cfg(any(feature = "png", feature = "pdf"))]
use resvg::usvg::Tree;

mod edge;
mod multigraph;
mod node;
mod options;
mod style;
mod subgraph;

const TYPST_PREFIX: &str = "__typst__";

#[cfg(feature = "png")]
const SCALING_FACTOR: f32 = 8.0;
#[cfg(feature = "pdf")]
const DPI: f32 = 400.0;

#[cfg(any(feature = "png", feature = "pdf"))]
static FONTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/fonts");

// -- public export
pub mod extras;

pub use edge::{ArrowKind, Edge};
pub use multigraph::Multigraph;
pub use node::{Node, NodeShape};
pub use options::*;
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
    pub fn from(content: String, width: f32, height: f32, style: &Style) -> Self {
        let header = format!("<svg viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">");
        let (mut defs, mut css_style) = (String::new(), String::new());

        // 1. add <defs>
        for section in style.get_utils().get_defs() {
            defs.push_str(section);
        }

        // 2. add <style>
        // 2.1 web fonts
        for webfont_url in style.get_utils().get_webfonts() {
            css_style += &format!("@import url(\"{webfont_url}\");\n");
        }
        // 2.2. background-color
        if style.get_fullscreen() {
            css_style += &format!("svg {{ background-color: {}; }}\n", style.get_background_color());
        }

        // return
        Self {
            content,
            defs,
            header,
            style: css_style,
        }
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
    pub fn save_svg<F: Display>(&self, filename: F) -> Result<()> {
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
    #[cfg(not(feature = "pdf"))]
    pub fn save_pdf<F: Display>(&self, _filename: F) -> Result<()> {
        anyhow::bail!("Feature 'pdf' is disabled");
    }
    #[cfg(feature = "pdf")]
    pub fn save_pdf<F: Display>(&self, filename: F) -> Result<()> {
        use svg2pdf::{ConversionOptions, PageOptions};

        let mut filepath = filename.to_string();

        // 1. create tree (from svg)
        let tree = self.create_tree().context("Error creating pixmap")?;

        // 3. Konverter til PDF
        let mut page_opt = PageOptions::default();
        page_opt.dpi = DPI;
        let pdf_data = svg2pdf::to_pdf(&tree, ConversionOptions::default(), page_opt).expect("Error creating PDF");

        // 5. append filextension if not exist
        if !filepath.ends_with(".pdf") {
            filepath += ".pdf";
        }

        // 6. add parent folders (if not exist)
        let path = Path::new(&filepath);
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        // 5. save to file
        std::fs::write(filepath, pdf_data).context("Error saving Graphplot PDF to file")
    }
    #[cfg(not(feature = "png"))]
    pub fn save_png<F: Display>(&self, _filename: F) -> Result<()> {
        anyhow::bail!("Feature 'png' is disabled");
    }
    #[cfg(feature = "png")]
    pub fn save_png<F: Display>(&self, filename: F) -> Result<()> {
        use resvg::{tiny_skia::Pixmap, usvg::Transform};
        let mut filepath = filename.to_string();

        // 1. create tree (from svg)
        let tree = self.create_tree().context("Error creating pixmap")?;

        // 2. get correct dimensions
        let size = tree.size();
        let width = (SCALING_FACTOR * size.width()).ceil() as u32;
        let height = (SCALING_FACTOR * size.height()).ceil() as u32;

        // 3. create pixelmap & render
        let mut pixmap = Pixmap::new(width, height).context("Error allocating pixmap")?;
        resvg::render(&tree, Transform::from_scale(SCALING_FACTOR, SCALING_FACTOR), &mut pixmap.as_mut());

        // 4. convert to png
        let png_data = pixmap.encode_png().context("Error rendering pixmap to png")?;

        // 5. append filextension if not exist
        if !filepath.ends_with(".png") {
            filepath += ".png";
        }

        // 6. add parent folders (if not exist)
        let path = Path::new(&filepath);
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        // 5. save to file
        std::fs::write(filepath, png_data).context("Error saving Graphplot PNG to file")
    }

    // -- helpers
    #[cfg(any(feature = "png", feature = "pdf"))]
    fn create_tree(&self) -> Result<Tree> {
        use resvg::usvg::{self, Options, Tree};
        use std::sync::Arc;
        let mut fontdb = usvg::fontdb::Database::new();

        // 1. get complete SVG
        let svg = self.to_complete_svg();

        // 2. load embedded fonts
        let mut opt = Options::default();
        Self::load_embedded_fonts(&mut fontdb);
        opt.fontdb = Arc::new(fontdb);

        // 3. parse complete svg
        let tree = Tree::from_data(svg.as_bytes(), &opt)?;

        // return
        Ok(tree)
    }
    #[cfg(any(feature = "png", feature = "pdf"))]
    fn load_embedded_fonts(db: &mut resvg::usvg::fontdb::Database) {
        // fn: recursively load sub-directories
        fn load_recursive(dir: &Dir, db: &mut resvg::usvg::fontdb::Database) {
            for file in dir.files() {
                db.load_font_data(file.contents().to_vec());
            }
            for subdir in dir.dirs() {
                load_recursive(subdir, db);
            }
        }
        load_recursive(&FONTS_DIR, db);
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
